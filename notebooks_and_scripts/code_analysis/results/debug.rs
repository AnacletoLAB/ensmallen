use super::*;
use vec_rand::sorted_unique_sub_sampling;

/// # Walk Queries
/// These are the queries that are used mainly in the random walk.
impl Graph {
    pub(crate) fn get_node_edges_and_destinations(
        &self,
        max_neighbours: Option<NodeT>,
        random_state: u64,
        node: NodeT,
    ) -> (EdgeT, EdgeT, Option<Vec<NodeT>>, Option<Vec<u64>>) {
        // We retrieve the range of edge ids, the minimum and maximum value.
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_by_source_node_id(node);

        // We check if subsampling is enabled and if so, if it makes sense:
        // that is, if the range of neighbours (max_edge_id-min_edge_id) is smaller
        // than the required sub-sampling we do not use it as it would be useless.
        if let Some(indices) = max_neighbours.and_then(|mn| {
            sorted_unique_sub_sampling(min_edge_id, max_edge_id, mn as u64, random_state).ok()
        }) {
            let destinations: Vec<NodeT> = match self
                .cached_destinations
                .as_ref()
                .and_then(|cds| cds.get(&node))
            {
                Some(dsts) => indices
                    .iter()
                    .map(|edge_id| dsts[(*edge_id - min_edge_id) as usize])
                    .collect(),
                None => indices
                    .iter()
                    .map(|edge_id| self.get_unchecked_destination_node_id_by_edge_id(*edge_id))
                    .collect(),
            };
            return (min_edge_id, max_edge_id, Some(destinations), Some(indices));
        }

        // If the destinations are stored explicitly because the time-memory tradeoff is enabled we are done.
        if self.destinations.is_some() {
            return (min_edge_id, max_edge_id, None, None);
        }

        // Finally if we are using the cache without sub-sampling
        let destinations = match self
            .cached_destinations
            .as_ref()
            .map_or(false, |cds| cds.contains_key(&node))
        {
            true => None,
            false => Some(self.iter_node_neighbours_ids(node).collect()),
        };
        (min_edge_id, max_edge_id, destinations, None)
    }

    /// Returns slice of destinations corresponding to given minmax edge ID and node.
    pub(crate) fn get_destinations_slice<'a>(
        &'a self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        node: NodeT,
        destinations: &'a Option<Vec<NodeT>>,
    ) -> &'a [NodeT] {
        match (&self.destinations, &self.cached_destinations, destinations) {
            (_, _, Some(dsts)) => &dsts.as_slice(),
            (Some(dsts), None, None) => &dsts[min_edge_id as usize..max_edge_id as usize],
            (None, Some(dsts), None) => dsts.get(&node).unwrap(),
            _ => unreachable!(
                "It is not possible to have both destinations and cached destinations at once."
            ),
        }
    }
}

use super::*;
use std::ops;

fn build_operator_graph_name(main: &Graph, other: &Graph, operator: String) -> String {
    format!("({} {} {})", main.name, operator, other.name)
}

/// Return graph composed of the two near-incompatible graphs.
///
/// The two graphs can have different nodes, edge types and node types.
/// These operators are slower than the generic integer operators since they
/// require a reverse mapping step.
///
/// # Arguments
///
/// * main: &Graph - The current graph instance.
/// * other: &Graph - The other graph.
/// * operator: String - The operator used.
/// * graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)> - Graph list for the operation.
/// * might_have_singletons: bool - Whether we expect the graph to have singletons.
/// * might_have_singletons_with_selfloops: bool - Whether we expect the graph to have singletons with self-loops.
/// * might_have_trap_nodes: bool - Whether we expect the graph to have trap nodes.
fn generic_string_operator(
    main: &Graph,
    other: &Graph,
    operator: String,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
    might_have_singletons: bool,
    might_have_singletons_with_selfloops: bool,
    might_have_trap_nodes: bool,
) -> Result<Graph, String> {
    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    let edges_iterator = graphs
        .iter()
        .flat_map(|(one, deny_graph, must_have_graph)| {
            one.iter_edge_with_type_and_weight(main.directed)
                .filter(move |(_, _, src_name, _, dst_name, _, edge_type_name, _)| {
                    // If the secondary graph is given
                    // we filter out the edges that were previously added to avoid
                    // introducing duplicates.
                    // TODO: handle None type edge types and avoid duplicating those!
                    if let Some(dg) = deny_graph {
                        return !dg.has_edge_with_type_by_node_names(
                            src_name,
                            dst_name,
                            edge_type_name.as_ref(),
                        );
                    }
                    if let Some(mhg) = must_have_graph {
                        return mhg.has_edge_with_type_by_node_names(
                            src_name,
                            dst_name,
                            edge_type_name.as_ref(),
                        );
                    }
                    true
                })
                .map(|(_, _, src_name, _, dst_name, _, edge_type_name, weight)| {
                    Ok((src_name, dst_name, edge_type_name, weight))
                })
        });

    // Chaining node types in a way that merges the information between
    // two node type sets where one of the two has some unknown node types
    let nodes_iterator = main
        .iter_nodes()
        .map(|(_, node_name, _, node_type_names)| {
            let node_type_names = match node_type_names {
                Some(ntns) => Some(ntns),
                None => other
                    .get_node_id_by_node_name(&node_name)
                    .ok()
                    .and_then(|node_id| other.get_node_type_name_by_node_id(node_id).unwrap()),
            };
            Ok((node_name, node_type_names))
        })
        .chain(
            other
                .iter_nodes()
                .filter_map(|(_, node_name, _, node_type_names)| {
                    match main.has_node_by_node_name(&node_name) {
                        true => None,
                        false => Some(Ok((node_name, node_type_names))),
                    }
                }),
        );

    Graph::from_string_unsorted(
        edges_iterator,
        Some(nodes_iterator),
        main.directed,
        false,
        build_operator_graph_name(main, other, operator),
        true,
        true,
        false,
        true,
        false,
        false,
        false,
        false,
        false,
        main.has_node_types(),
        main.has_edge_types(),
        main.has_weights(),
        might_have_singletons,
        might_have_singletons_with_selfloops,
        might_have_trap_nodes,
    )
}

/// Return graph composed of the two compatible graphs.
///
/// The two graphs CANNOT have different nodes, edge types and node types.
/// These operators are faster than the generic string operators since they
/// do NOT require a reverse mapping step.
///
/// # Arguments
///
/// * main: &Graph - The current graph instance.
/// * other: &Graph - The other graph.
/// * operator: String - The operator used.
/// * graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)> - Graph list for the operation.
/// * might_have_singletons: bool - Whether we expect the graph to have singletons.
/// * might_have_singletons_with_selfloops: bool - Whether we expect the graph to have singletons with self-loops.
/// * might_have_trap_nodes: bool - Whether we expect the graph to have trap nodes.
fn generic_integer_operator(
    main: &Graph,
    other: &Graph,
    operator: String,
    graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
    might_have_singletons: bool,
    might_have_singletons_with_selfloops: bool,
    might_have_trap_nodes: bool,
) -> Result<Graph, String> {
    // one: left hand side of the operator
    // deny_graph: right hand edges "deny list"
    // must_have_graph: right hand edges "must have list
    let edges_iterator = graphs
        .iter()
        .flat_map(|(one, deny_graph, must_have_graph)| {
            one.iter_edge_with_type_and_weight_ids(main.directed)
                .filter(move |(_, src, dst, edge_type, _)| {
                    // If the secondary graph is given
                    // we filter out the edges that were previously added to avoid
                    // introducing duplicates.
                    if let Some(dg) = deny_graph {
                        return !dg.has_edge_with_type_by_node_ids(*src, *dst, *edge_type);
                    }
                    if let Some(mhg) = must_have_graph {
                        return mhg.has_edge_with_type_by_node_ids(*src, *dst, *edge_type);
                    }
                    true
                })
                .map(|(_, src, dst, edge_type, weight)| Ok((src, dst, edge_type, weight)))
        });

    let node_types = match (&main.node_types, &other.node_types) {
        (Some(mnts), Some(onts)) => Some(match mnts == onts {
            true => mnts.clone(),
            false => {
                let mut main_node_types = mnts.ids.clone();
                main_node_types
                    .iter_mut()
                    .zip(onts.ids.iter())
                    .for_each(|(mid, oid)| {
                        if mid.is_none() {
                            *mid = oid.clone();
                        }
                    });
                NodeTypeVocabulary::from_structs(main_node_types, Some(mnts.vocabulary.clone()))
                    .unwrap()
            }
        }),
        (Some(mnts), _) => Some(mnts.clone()),
        (_, Some(onts)) => Some(onts.clone()),
        _ => None,
    };

    Graph::from_integer_unsorted(
        edges_iterator,
        main.nodes.clone(),
        node_types,
        main.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
        main.directed,
        build_operator_graph_name(main, other, operator),
        false,
        main.has_edge_types(),
        main.has_weights(),
        false,
        might_have_singletons,
        might_have_singletons_with_selfloops,
        might_have_trap_nodes,
    )
}

impl<'a, 'b> Graph {
    pub fn validate_operator_terms(&self, other: &'b Graph) -> Result<(), String> {
        if self.directed != other.directed {
            return Err(String::from(
                "The graphs must either be both directed or undirected.",
            ));
        }

        if self.has_weights() != other.has_weights() {
            return Err(String::from(
                "Both graphs need to have weights or neither can.",
            ));
        }

        if self.has_node_types() != other.has_node_types() {
            return Err(String::from(
                "Both graphs need to have node types or neither can.",
            ));
        }

        if self.has_edge_types() != other.has_edge_types() {
            return Err(String::from(
                "Both graphs need to have edge types or neither can.",
            ));
        }

        Ok(())
    }
}

impl Graph {
    /// Return true if the graphs are compatible.
    pub(crate) fn is_compatible(&self, other: &Graph) -> Result<bool, String> {
        self.validate_operator_terms(other)?;
        if self.nodes != other.nodes {
            return Ok(false);
        }
        if let (Some(snts), Some(onts)) = (&self.node_types, &other.node_types) {
            if snts.vocabulary != onts.vocabulary {
                return Ok(false);
            }
        }
        if let (Some(sets), Some(oets)) = (&self.edge_types, &other.edge_types) {
            if sets.vocabulary != oets.vocabulary {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(crate) fn generic_operator(
        &self,
        other: &Graph,
        operator: String,
        graphs: Vec<(&Graph, Option<&Graph>, Option<&Graph>)>,
        might_have_singletons: bool,
        might_have_singletons_with_selfloops: bool,
        might_have_trap_nodes: bool,
    ) -> Result<Graph, String> {
        match self.is_compatible(other)? {
            true => generic_integer_operator(
                self,
                other,
                operator,
                graphs,
                might_have_singletons,
                might_have_singletons_with_selfloops,
                might_have_trap_nodes,
            ),
            false => generic_string_operator(
                self,
                other,
                operator,
                graphs,
                might_have_singletons,
                might_have_singletons_with_selfloops,
                might_have_trap_nodes,
            ),
        }
    }
}

impl<'a, 'b> ops::BitOr<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return graph composed of the two graphs.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be summed.
    ///
    fn bitor(self, other: &'b Graph) -> Result<Graph, String> {
        self.generic_operator(
            other,
            "|".to_owned(),
            vec![(self, None, None), (other, Some(self), None)],
            // TODO: it is possible to make the following more precise!
            self.has_singletons() || other.has_singletons(),
            // TODO: it is possible to make the following more precise!
            self.has_singleton_nodes_with_self_loops()
                || other.has_singleton_nodes_with_self_loops(),
            // TODO: it is possible to make the following more precise!
            self.has_trap_nodes() || other.has_trap_nodes(),
        )
    }
}

impl<'a, 'b> ops::BitXor<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return graph composed of the two graphs.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be summed.
    ///
    fn bitxor(self, other: &'b Graph) -> Result<Graph, String> {
        self.generic_operator(
            self,
            "^".to_owned(),
            vec![(self, Some(other), None), (other, Some(self), None)],
            true,
            // TODO: it is possible to make the following more precise!
            self.has_selfloops() || other.has_selfloops(),
            true,
        )
    }
}

impl<'a, 'b> ops::Sub<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return subtraction for graphs objects.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be subtracted.
    ///
    fn sub(self, other: &'b Graph) -> Result<Graph, String> {
        self.generic_operator(
            other,
            "-".to_owned(),
            vec![(self, Some(other), None)],
            true,
            self.has_selfloops(),
            true,
        )
    }
}

impl<'a, 'b> ops::BitAnd<&'b Graph> for &'a Graph {
    type Output = Result<Graph, String>;
    /// Return graph obtained from the intersection of the two graph.
    ///
    /// The two graphs must have the same nodes, node types and edge types.
    ///
    /// # Arguments
    ///
    /// * other: Graph - Graph to be subtracted.
    ///
    fn bitand(self, other: &'b Graph) -> Result<Graph, String> {
        self.generic_operator(
            other,
            "&".to_owned(),
            vec![(self, None, Some(other))],
            true,
            self.has_selfloops() && other.has_selfloops(),
            true,
        )
    }
}

use super::*;

/// # Unchecked Queries
/// The naming convection for unchecked methods follows `get_unchecked_X_by_Y`.
impl Graph {
    /// Returns the name of the node passed and HORRIBLY PANIC if the id is out
    /// of range.
    pub(crate) fn get_unchecked_node_name_by_node_id(&self, node_id: NodeT) -> String {
        self.nodes.unchecked_translate(node_id)
    }

    /// Returns option with the edge type of the given edge id.
    pub(crate) fn get_unchecked_edge_type_by_edge_id(&self, edge_id: EdgeT) -> Option<EdgeTypeT> {
        self.edge_types
            .as_ref()
            .and_then(|ets| ets.ids[edge_id as usize])
    }

    /// Returns option with the weight of the given edge id.
    pub(crate) fn get_unchecked_weight_by_edge_id(&self, edge_id: EdgeT) -> Option<WeightT> {
        self.weights.as_ref().map(|ws| ws[edge_id as usize])
    }

    /// Returns option with the node type of the given node id.
    pub(crate) fn get_unchecked_node_type_id_by_node_id(
        &self,
        node_id: NodeT,
    ) -> Option<Vec<NodeTypeT>> {
        self.node_types
            .as_ref()
            .and_then(|nts| nts.ids[node_id as usize].clone())
    }

    /// Returns node id raising a panic if used unproperly.
    pub(crate) fn get_unchecked_node_id_by_node_name(&self, node_name: &str) -> NodeT {
        *self.nodes.get(node_name).unwrap()
    }

    /// Return edge type ID corresponding to the given edge type name.
    pub(crate) fn get_unchecked_edge_type_id_by_edge_type_name(
        &self,
        edge_type_name: &str,
    ) -> Option<EdgeTypeT> {
        self.edge_types
            .as_ref()
            .and_then(|ets| ets.get(edge_type_name).copied())
    }

    /// Return edge type ID corresponding to the given edge type name
    /// raising panic if edge type ID does not exists in current graph.
    pub(crate) fn get_unchecked_edge_type_name_by_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Option<String> {
        match (&self.edge_types, edge_type_id) {
            (Some(ets), Some(et)) => Some(ets.unchecked_translate(et)),
            _ => None,
        }
    }

    /// Return number of edges of the given edge type without checks.
    ///
    /// # Arguments
    ///
    /// * edge_type: Option<EdgeTypeT> - The edge type to retrieve count of.
    ///
    pub(crate) fn get_unchecked_edge_count_by_edge_type_id(
        &self,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        match (&self.edge_types, edge_type) {
            (Some(ets), None) => ets.get_unknown_count(),
            (Some(ets), Some(et)) => ets.counts[et as usize],
            _ => unreachable!("The current graph instance does not have edge types!"),
        }
    }

    /// Return number of nodes of the given node type without checks.
    ///
    /// # Arguments
    ///
    /// * node_type: Option<NodeTypeT> - The node type to retrieve count of.
    ///
    pub(crate) fn get_unchecked_node_count_by_node_type_id(
        &self,
        node_type: Option<NodeTypeT>,
    ) -> NodeT {
        match (&self.node_types, node_type) {
            (Some(nts), None) => nts.get_unknown_count(),
            (Some(nts), Some(nt)) => nts.counts[nt as usize],
            _ => unreachable!("The current graph instance does not have node types!"),
        }
    }

    /// Return (subsampled) vector of destinations of given node.
    ///
    /// If the max neighbours parameter is given, and is smaller than the
    /// number of the neighbours of the given node, the subsampling
    /// mechanism is given.
    ///
    /// # Arguments
    /// `node`: NodeT - Node whose neighbours are to return.
    /// `random_state`: u64 - Random state to subsample neighbours.
    /// `max_neighbours`: &Option<NodeT> - Optionally number of neighbours to consider.
    pub(crate) fn get_unchecked_node_destinations_by_node_id(
        &self,
        node: NodeT,
        random_state: u64,
        max_neighbours: Option<NodeT>,
    ) -> Vec<NodeT> {
        let (min_edge_id, max_edge_id, destinations, _) =
            self.get_node_edges_and_destinations(max_neighbours, random_state, node);
        self.get_destinations_slice(min_edge_id, max_edge_id, node, &destinations)
            .to_owned()
    }

    /// Return edge ID without any checks for given tuple of nodes and edge type.
    ///
    /// This method will cause a panic if used improperly when it is not certain
    /// that the edge exists.
    ///
    /// # Arguments
    /// `src`: NodeT - Source node of the edge.
    /// `dst`: NodeT - Destination node of the edge.
    /// `edge_type`: Option<EdgeTypeT> - Edge Type of the edge.
    pub(crate) fn get_unchecked_edge_id_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> EdgeT {
        self.edge_types.as_ref().map_or_else(
            || self.get_unchecked_edge_id_from_tuple(src, dst),
            |ets| {
                self.get_unchecked_edge_ids_range(src, dst)
                    // The vectors of the edge types can only have one element.
                    .find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
                    .unwrap()
            },
        )
    }

    /// Returns range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    ///
    /// * `src` - Source node of the edge.
    /// * `dst` - Destination node of the edge.
    ///
    pub(crate) fn get_unchecked_edge_ids_range(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> impl Iterator<Item = EdgeT> {
        let (min_edge_id, max_edge_id) = self.get_unchecked_minmax_edge_ids_by_node_ids(src, dst);
        min_edge_id..max_edge_id
    }

    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_minmax_edge_ids_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> (EdgeT, EdgeT) {
        (
            self.get_unchecked_edge_id_from_tuple(src, dst),
            self.get_unchecked_edge_id_from_tuple(src, dst + 1),
        )
    }

    /// Return the number of edges between the given source and destination nodes.
    ///
    /// This might be thought as the degree of an edge in a multigraph.
    /// On non-multigraph this trivially return 1 on existing edges and 0 on
    /// the non-existing ones.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_unchecked_edge_degreee_by_node_ids(&self, src: NodeT, dst: NodeT) -> EdgeT {
        let (min_edge_id, max_edge_id) = self.get_unchecked_minmax_edge_ids_by_node_ids(src, dst);
        max_edge_id - min_edge_id
    }
}

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
    /// * walk_length: usize - Maximal walk_length of the walk.
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
    /// * walk_length: NodeT - Maximal walk_length of the walk.
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
    /// * iterations: Option<NodeT> - whether to show the loading bar or not.
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
    /// * max_neighbours: Option<NodeT> - Number of neighbours to consider for each extraction.
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
    /// assert!(parameters.set_dense_node_mapping(Some(ppi.get_dense_node_mapping())).validate(&ppi).is_ok());
    /// ```
    /// Two different graphs, like Cora and STRING, are not remappable:
    /// ```rust
    /// # use graph::walks_parameters::WalksParameters;
    /// # let cora = graph::test_utilities::load_cora().unwrap();
    /// # let ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let mut parameters = WalksParameters::new(32).unwrap();
    /// assert!(parameters.set_dense_node_mapping(Some(ppi.get_dense_node_mapping())).validate(&cora).is_err());
    /// ```
    ///
    pub fn validate(&self, graph: &Graph) -> Result<(), String> {
        if let Some(dense_node_mapping) = &self.dense_node_mapping {
            if !graph
                .iter_unique_sources()
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

use super::*;

use indicatif::ProgressIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::{collections::HashSet, sync::atomic::AtomicU32};
use vec_rand::xorshift::xorshift as rand_u64;

const NOT_PRESENT: u32 = u32::MAX;

/// # Implementation of algorithms relative to trees.
///
/// # Definitions
/// - **Self-loops**: Edges with source equal to the destination.
/// - **Singleton**: A node with no incident edges, (self-loops are not considered).
/// - **Spanning Tree**: A set of edges that allows to build a path between every
///     node in the graph. For a graph with n nodes the spanning tree will have n - 1 edges.
/// - **Spanning Arborescence**: is the generalizzation of the spanning tree for graphs
///     with multiple components. Being a tree it trivially contains no self-loops.
///     For a grpah with n nodes and c components the spanning arborescence will have
///     n - c edges.
/// - **Component**: Set of nodes in which any two vertices in it are connected to
///     each other by paths. A singleton is a component and so is a singleton with a
///     self-loop.
impl Graph {
    fn iter_edges_from_random_state(
        &self,
        random_state: u64,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        let edges_number = self.get_directed_edges_number();
        // We execute two times the xorshift to improve the randomness of the seed.
        let updated_random_state = rand_u64(rand_u64(random_state ^ SEED_XOR as u64));
        (updated_random_state..edges_number + updated_random_state).filter_map(move |i| {
            let edge_id = i % edges_number;
            let (src, dst) = self.get_node_ids_from_edge_id(edge_id);
            match src == dst || !self.directed && src > dst {
                true => None,
                false => Some((edge_id, src, dst)),
            }
        })
    }

    fn iter_on_edges_with_preference<'a>(
        &'a self,
        random_state: u64,
        unwanted_edge_types: &'a Option<HashSet<Option<EdgeTypeT>>>,
        verbose: bool,
    ) -> impl Iterator<Item = (NodeT, NodeT)> + 'a {
        let pb = get_loading_bar(
            verbose,
            format!("Building random spanning tree for {}", self.name).as_ref(),
            self.get_directed_edges_number() as usize,
        );
        let result: Box<dyn Iterator<Item = (NodeT, NodeT)>> = if let (Some(uet), _) =
            (unwanted_edge_types, &self.edge_types)
        {
            Box::new(
                self.iter_edges_from_random_state(random_state)
                    .filter_map(move |(edge_id, src, dst)| {
                        if uet.contains(&self.get_unchecked_edge_type_by_edge_id(edge_id)) {
                            return None;
                        }
                        Some((src, dst))
                    })
                    .chain(self.iter_edges_from_random_state(random_state).filter_map(
                        move |(edge_id, src, dst)| {
                            if !uet.contains(&self.get_unchecked_edge_type_by_edge_id(edge_id)) {
                                return None;
                            }
                            Some((src, dst))
                        },
                    )),
            )
        } else {
            Box::new(
                self.iter_edges_from_random_state(random_state)
                    .map(|(_, src, dst)| (src, dst)),
            )
        };

        result.progress_with(pb)
    }

    /// Returns set of edges composing a spanning tree and connected components.
    ///
    /// If the graph is composed of a single node with one or more self-loops,
    /// we consider such a graph as a graph with an empty spanning tree, with
    /// a single component of size one.
    ///
    /// # Arguments
    ///
    /// `edges` - Iterator for the edges to explore. If sorted, computed a minimum spanning tree.
    ///
    /// # Returns
    /// Tuple with:
    ///     - Set of the edges
    ///     - Vector of the nodes components
    ///     - Total components number
    ///     - Minimum component size
    ///     - Maximum component size
    pub(crate) fn kruskal<'a>(
        &self,
        edges: impl Iterator<Item = (NodeT, NodeT)> + 'a,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        if !self.has_nodes() {
            return (HashSet::new(), Vec::new(), 0, 0, 0);
        }
        if !self.has_edges() {
            return (
                HashSet::new(),
                (0..self.get_nodes_number()).collect(),
                self.get_nodes_number(),
                1,
                1,
            );
        }

        let nodes_number = self.get_nodes_number() as usize;
        let mut tree = HashSet::with_capacity(self.get_nodes_number() as usize);
        let mut components = vec![NOT_PRESENT; nodes_number];
        let mut merged_component_number = 0;
        let mut component_sizes: Vec<NodeT> = Vec::new();
        let mut components_remapping: Vec<NodeT> = Vec::new();
        let mut max_component_size: NodeT = 0;
        let mut min_component_size = NodeT::MAX;

        // When there are singleton nodes with self-loops,
        // which is an arguability weird feature of some graphs,
        // Kruskal fails to identify them because by definition
        // a tree cannot contain self-loop.
        // We call these nodes with one or more self-loops
        // (in the case of a multigraph) `singletons with self-loops` for lack of
        // a better term. These nodes are treated as nodes in their own
        // component and their edges (the self-loops) are not added to the tree.
        if self.has_singletons() || self.has_singleton_nodes_with_self_loops() {
            min_component_size = 1;
            max_component_size = 1;
            (0..self.get_nodes_number())
                .filter(|node_id| {
                    self.is_singleton_by_node_id(*node_id).unwrap()
                        || self.is_singleton_with_self_loops_by_node_id(*node_id)
                })
                .for_each(|node_id| {
                    components[node_id as usize] = component_sizes.len() as NodeT;
                    components_remapping.push(component_sizes.len() as NodeT);
                    component_sizes.push(1);
                });
        }

        edges.for_each(|(src, dst)| {
            // If this is a self-loop we skip it.
            if src == dst {
                return;
            }
            let src_component = components[src as usize];
            let dst_component = components[dst as usize];
            match (src_component == NOT_PRESENT, dst_component == NOT_PRESENT) {
                // If neither nodes have a component, they must be inserted
                // both in the components vector and in the tree.
                // The edge must be added to the three.
                (true, true) => {
                    let component_number = components_remapping.len() as NodeT;
                    components[src as usize] = component_number;
                    components[dst as usize] = component_number;
                    components_remapping.push(component_number);
                    component_sizes.push(2);
                    max_component_size = max_component_size.max(2);
                    tree.insert((src, dst));
                }
                // If both nodes have a component, the two components must be merged
                // if they are not the same one.
                // The edge must be added to the three.
                // The components mapping must be updated and afterwards the other nodes
                // must be updated accordingly to this update.
                (false, false) => {
                    if src_component == dst_component {
                        return;
                    }
                    let src_component = components_remapping[src_component as usize];
                    let dst_component = components_remapping[dst_component as usize];
                    components[src as usize] = dst_component;
                    components[dst as usize] = dst_component;
                    if src_component == dst_component {
                        return;
                    }
                    let (min_component, max_component) = match src_component < dst_component {
                        true => (src_component, dst_component),
                        false => (dst_component, src_component),
                    };
                    merged_component_number += 1;
                    component_sizes[min_component as usize] +=
                        component_sizes[max_component as usize];
                    max_component_size =
                        max_component_size.max(component_sizes[min_component as usize]);

                    components_remapping
                        .iter_mut()
                        .enumerate()
                        .for_each(|(comp, remapped)| {
                            if *remapped == max_component {
                                *remapped = min_component;
                                component_sizes[comp] = 0;
                            }
                        });
                    tree.insert((src, dst));
                }
                // If only one node has a component, the second model must be added.
                _ => {
                    let (component_number, not_inserted_node) = match src_component == NOT_PRESENT {
                        true => (dst_component, src),
                        false => (src_component, dst),
                    };
                    let component_number = components_remapping[component_number as usize];
                    component_sizes[component_number as usize] += 1;
                    max_component_size =
                        max_component_size.max(component_sizes[component_number as usize]);
                    components[not_inserted_node as usize] = component_number as NodeT;
                    tree.insert((src, dst));
                }
            };
        });

        // Remapping components to a dense remapping
        let mut state = 0;
        for i in 0..components_remapping.len() {
            if components_remapping[i] >= state {
                components_remapping[i] = state;
                state += 1;
            } else {
                components_remapping[i] = components_remapping[components_remapping[i] as usize];
            }
        }

        components.par_iter_mut().for_each(|remapped| {
            *remapped = components_remapping[*remapped as usize];
        });

        let components_number = component_sizes.len() - merged_component_number;

        // If the minimum component size is still bigger than one
        // that is, we do not know alredy that there is a singleton
        // we need to compute it.
        if min_component_size > 1 {
            min_component_size = component_sizes
                .into_par_iter()
                .filter(|c| *c != 0)
                .min()
                .unwrap();
        }

        (
            tree,
            components,
            components_number as NodeT,
            min_component_size,
            max_component_size,
        )
    }

    /// Returns set of edges composing a spanning tree and connected components.
    ///
    /// The spanning tree is NOT minimal.
    /// The given random_state is NOT the root of the tree.
    ///
    /// # Arguments
    ///
    /// * `random_state`:NodeT - The random_state to use for the holdout,
    /// * `include_all_edge_types`: bool - whether to include all the edges between two nodes.
    /// * `unwanted_edge_types`: &Option<HashSet<EdgeTypeT>> - Which edge types id to try to avoid.
    /// * `verbose`: bool - whether to show a loading bar or not.
    ///
    pub fn random_spanning_arborescence_kruskal(
        &self,
        random_state: EdgeT,
        unwanted_edge_types: &Option<HashSet<Option<EdgeTypeT>>>,
        verbose: bool,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        self.kruskal(self.iter_on_edges_with_preference(random_state, unwanted_edge_types, verbose))
    }

    pub fn spanning_arborescence_kruskal(
        &self,
        verbose: bool,
    ) -> (HashSet<(NodeT, NodeT)>, Vec<NodeT>, NodeT, NodeT, NodeT) {
        let pb = get_loading_bar(
            verbose,
            &format!(
                "Computing spanning arborescence with Kruskal for {}",
                self.get_name()
            ),
            self.get_unique_edges_number() as usize,
        );
        self.kruskal(self.iter_unique_edges(self.directed).progress_with(pb))
    }

    /// Returns set of edges composing a spanning tree.
    ///
    /// This is the implementaiton of [A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors (SMPs)](https://smartech.gatech.edu/bitstream/handle/1853/14355/GT-CSE-06-01.pdf)
    /// by David A. Bader and Guojing Cong.
    pub fn spanning_arborescence(
        &self,
        verbose: bool,
    ) -> Result<(usize, impl Iterator<Item = (NodeT, NodeT)> + '_), String> {
        if self.directed {
            return Err(
                "The spanning arborescence from Bader et al. algorithm only works for undirected graphs!".to_owned(),
            );
        }
        let nodes_number = self.get_nodes_number() as usize;
        let mut parents = vec![NOT_PRESENT; nodes_number];
        let cpu_number = rayon::current_num_threads();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_number)
            .build()
            .unwrap();
        let shared_stacks: Arc<Vec<Mutex<Vec<NodeT>>>> = Arc::from(
            (0..std::cmp::max(cpu_number - 1, 1))
                .map(|_| Mutex::from(Vec::new()))
                .collect::<Vec<Mutex<Vec<NodeT>>>>(),
        );
        let active_nodes_number = AtomicUsize::new(0);
        let completed = AtomicBool::new(false);
        let total_inserted_edges = AtomicUsize::new(0);
        let thread_safe_parents = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut parents),
        };

        // since we were able to build a stub tree with cpu.len() leafs,
        // we spawn the treads and make anyone of them build the sub-trees.
        pool.scope(|s| {
            // for each leaf of the previous stub tree start a DFS keeping track
            // of which nodes we visited and updating accordingly the parents vector.
            // the nice trick here is that, since all the leafs are part of the same tree,
            // if two processes find the same node, we don't care which one of the two take
            // it so we can proceed in a lockless fashion (and maybe even without atomics
            // if we manage to remove the colors vecotr and only keep the parents one)
            s.spawn(|_| {
                let pb = get_loading_bar(
                    verbose,
                    format!("Computing spanning tree of graph {}", self.get_name()).as_ref(),
                    nodes_number,
                );
                let parents = thread_safe_parents.value.get();
                (0..nodes_number).progress_with(pb).for_each(|src| {
                    unsafe {
                        // If the node has already been explored we skip ahead.
                        if (*parents)[src] != NOT_PRESENT {
                            return;
                        }
                    }
                    unsafe {
                        // find the first not explored node (this is guardanteed to be in a new component)
                        if self.has_singletons()
                            && self.is_singleton_by_node_id(src as NodeT).unwrap()
                        {
                            // We set singletons as self-loops for now.
                            (*parents)[src] = src as NodeT;
                            return;
                        }
                    }
                    loop {
                        unsafe {
                            if (*parents)[src] != NOT_PRESENT {
                                break;
                            }
                        }
                        if active_nodes_number.load(Ordering::SeqCst) == 0 {
                            unsafe {
                                if (*parents)[src] != NOT_PRESENT {
                                    break;
                                }
                            }
                            unsafe {
                                (*parents)[src] = src as NodeT;
                            }
                            shared_stacks[0].lock().unwrap().push(src as NodeT);
                            active_nodes_number.fetch_add(1, Ordering::SeqCst);
                            break;
                        }
                    }
                });
                completed.store(true, Ordering::SeqCst);
            });
            (0..shared_stacks.len()).for_each(|_| {
                s.spawn(|_| 'outer: loop {
                    let thread_id = rayon::current_thread_index().unwrap();
                    let src = 'inner: loop {
                        {
                            for mut stack in (thread_id..(shared_stacks.len() + thread_id))
                                .map(|id| shared_stacks[id % shared_stacks.len()].lock().unwrap())
                            {
                                if let Some(src) = stack.pop() {
                                    break 'inner src;
                                }
                            }

                            if completed.load(Ordering::SeqCst) {
                                break 'outer;
                            }
                        }
                    };
                    let parents = thread_safe_parents.value.get();
                    self.iter_node_neighbours_ids(src).for_each(|dst| unsafe {
                        if (*parents)[dst as usize] == NOT_PRESENT {
                            (*parents)[dst as usize] = src;
                            total_inserted_edges.fetch_add(1, Ordering::SeqCst);
                            active_nodes_number.fetch_add(1, Ordering::SeqCst);
                            shared_stacks[rand_u64(dst as u64) as usize % shared_stacks.len()]
                                .lock()
                                .unwrap()
                                .push(dst);
                        }
                    });
                    active_nodes_number.fetch_sub(1, Ordering::SeqCst);
                });
            });
        });

        // convert the now completed parents vector to a list of tuples representing the edges
        // of the spanning arborescense.
        Ok((
            // Number of edges inserted
            total_inserted_edges.load(Ordering::SeqCst),
            // Return an iterator over all the edges in the spanning arborescence
            (0..self.get_nodes_number()).filter_map(move |src| {
                let dst = parents[src as usize];
                // If the edge is NOT registered as a self-loop
                // which may happen when dealing with singletons
                // or the root nodes, we return the edge.
                if src != dst {
                    return Some((src, dst));
                }
                None
            }),
        ))
    }

    /// Compute the connected components building in parallel a spanning tree using [bader's algorithm](https://www.sciencedirect.com/science/article/abs/pii/S0743731505000882).
    /// **This works only for undirected graphs.**
    ///
    /// This method is **not thread save and not deterministic** but by design of the algorithm this
    /// shouldn't matter but if we will encounter non-detemristic bugs here is where we want to look.
    ///
    /// Returns (Components membership, components number, size of the smallest components, size of the biggest components).
    /// We assign to each node the index of its component, so nodes in the same components will have the same index.
    /// This component index is the returned Components membership vector.
    ///
    /// Example:
    /// ```rust
    ///  # #![feature(impl_trait_in_bindings)]
    ///  # use graph::Graph;
    ///  // Graph is a weightless graph with the edges
    ///  // [(0, 1), (1, 4), (2, 3)]
    ///  # let edge: Vec<Result<(String, String, Option<String>, Option<f32>), String>> = vec![
    ///  #        Ok(("0".to_string(), "1".to_string(), None, None)),
    ///  #        Ok(("1".to_string(), "4".to_string(), None, None)),
    ///  #        Ok(("2".to_string(), "3".to_string(), None, None)),
    ///  #     ];
    ///  #
    ///  # let nodes = None.map(|x: Vec<Result<(String, Option<Vec<String>>), String>>| x.into_iter());
    ///  #
    ///  # let graph = Graph::from_string_unsorted(
    ///  #     edge.into_iter(),
    ///  #     nodes,      // nodes
    ///  #     false,     // directed
    ///  #     false,      // directe edge list
    ///  #     "test graph",// name
    ///  #     false,     // ignore_duplicated_nodes
    ///  #     true,     // node_list_is_correct
    ///  #     false,     // ignore_duplicated_nodes
    ///  #     true,     // node_list_is_correct
    ///  #     false,     // verbose
    ///  #     false,     // numeric_edge_types_ids
    ///  #     false,     // numeric_node_ids
    ///  #     false,     // numeric_edge_node_ids
    ///  #     false,     // numeric_node_types_ids
    ///  #     false,     // has_node_types
    ///  #     false,     // has_edge_types
    ///  #     false,     // has_weights
    ///  #     true,
    ///  #     true,
    ///  #     true,
    ///  # ).unwrap();
    /// let (components, number_of_components, smallest, biggest) =
    ///     graph.connected_components(false).unwrap();
    ///
    /// //   nodes names:       0  1  4  2  3
    /// assert_eq!(components, [0, 0, 0, 1, 1].to_vec());
    ///
    /// assert_eq!(number_of_components, 2);
    /// assert_eq!(smallest, 2); // the size of the smallest component
    /// assert_eq!(biggest, 3);  // the size of the biggest component
    /// ```
    pub fn connected_components(
        &self,
        verbose: bool,
    ) -> Result<(Vec<NodeT>, NodeT, NodeT, NodeT), String> {
        if self.directed {
            return Err(
                "The connected components algorithm only works for undirected graphs!".to_owned(),
            );
        }
        if !self.has_nodes() {
            return Ok((Vec::new(), 0, 0, 0));
        }
        if self.get_edges_number() == 0 {
            return Ok((
                (0..self.get_nodes_number()).collect(),
                self.get_nodes_number(),
                1,
                1,
            ));
        }
        let components = (0..self.get_nodes_number())
            .map(|_| AtomicU32::new(NOT_PRESENT))
            .collect::<Vec<_>>();
        let mut min_component_size: NodeT = NodeT::MAX;
        let mut max_component_size: NodeT = 0;
        let mut components_number: NodeT = 0;
        let cpu_number = rayon::current_num_threads();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_number)
            .build()
            .unwrap();
        let shared_stacks: Arc<Vec<Mutex<Vec<NodeT>>>> = Arc::from(
            (0..std::cmp::max(cpu_number - 1, 1))
                .map(|_| Mutex::from(Vec::new()))
                .collect::<Vec<Mutex<Vec<NodeT>>>>(),
        );
        let active_nodes_number = AtomicUsize::new(0);
        let current_component_size = AtomicU32::new(0);
        let completed = AtomicBool::new(false);
        let thread_safe_min_component_size = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut min_component_size),
        };
        let thread_safe_max_component_size = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut max_component_size),
        };
        let thread_safe_components_number = ThreadSafe {
            value: std::cell::UnsafeCell::new(&mut components_number),
        };

        // since we were able to build a stub tree with cpu.len() leafs,
        // we spawn the treads and make anyone of them build the sub-trees.
        pool.scope(|s| {
            // for each leaf of the previous stub tree start a DFS keeping track
            // of which nodes we visited and updating accordingly the components vector.
            // the nice trick here is that, since all the leafs are part of the same tree,
            // if two processes find the same node, we don't care which one of the two take
            // it so we can proceed in a lockless fashion (and maybe even without atomics
            // if we manage to remove the colors vecotr and only keep the components one)
            s.spawn(|_| {
                let pb = get_loading_bar(
                    verbose,
                    format!(
                        "Computing connected components of graph {}",
                        self.get_name()
                    )
                    .as_ref(),
                    self.get_nodes_number() as usize,
                );
                let min_component_size = thread_safe_min_component_size.value.get();
                let max_component_size = thread_safe_max_component_size.value.get();
                let components_number = thread_safe_components_number.value.get();
                (0..self.get_nodes_number())
                    .progress_with(pb)
                    .for_each(|src| {
                        // If the node has already been explored we skip ahead.
                        if components[src as usize].load(Ordering::Relaxed) != NOT_PRESENT {
                            return;
                        }

                        // find the first not explored node (this is guardanteed to be in a new component)
                        if self.has_singletons()
                            && (self.is_singleton_by_node_id(src).unwrap()
                                || self.is_singleton_with_self_loops_by_node_id(src))
                        {
                            // We set singletons as self-loops for now.
                            unsafe {
                                components[src as usize]
                                    .store(**components_number, Ordering::Relaxed);
                                **components_number += 1;
                                **min_component_size = 1;
                                **max_component_size = (**max_component_size).max(1);
                            }
                            return;
                        }

                        loop {
                            // if the node has been now mapped to a component by anyone of the
                            // parallel threads, move on to the next node.
                            if components[src as usize].load(Ordering::Relaxed) != NOT_PRESENT {
                                break;
                            }
                            // Otherwise, Check if the parallel threads are finished
                            // and are all waiting for a new node to explore.
                            // In that case add the currently not explored node to the
                            // work stack of the first thread.
                            if active_nodes_number.load(Ordering::Relaxed) == 0 {
                                // The check here might seems redundant but its' needed
                                // to prevent data races.
                                //
                                // If the last parallel thread finishes its stack between the
                                // presence check above and the active nodes numbers check
                                // the src node will never increase the component size and thus
                                // leading to wrong results.
                                if components[src as usize].load(Ordering::Relaxed) != NOT_PRESENT {
                                    break;
                                }
                                let ccs = current_component_size.swap(1, Ordering::Relaxed) as NodeT;
                                unsafe {
                                    **max_component_size = (**max_component_size).max(ccs);
                                    if ccs > 1 {
                                        **min_component_size = (**min_component_size).min(ccs);
                                    }
                                    components[src as usize]
                                        .store(**components_number, Ordering::Relaxed);
                                    **components_number += 1;
                                }
                                active_nodes_number.fetch_add(1, Ordering::Relaxed);
                                shared_stacks[0].lock().unwrap().push(src);
                                break;
                            }
                            // Otherwise, Loop until the parallel threads are finished.
                        }
                    });
                unsafe {
                    let ccs = current_component_size.load(Ordering::Relaxed);
                    **max_component_size = (**max_component_size).max(ccs);
                    if ccs > 1 {
                        **min_component_size = (**min_component_size).min(ccs);
                    }
                }
                completed.store(true, Ordering::Relaxed);
            });

            // Spawn the parallel threads that handle the components mapping,
            // these threads use work-stealing, meaning that if their stack is empty,
            // they will steal nodes from the stack of another random thread.
            (0..shared_stacks.len()).for_each(|_| {
                s.spawn(|_| 'outer: loop {
                    // get the id, we use this as an idex for the stacks vector.
                    let thread_id = rayon::current_thread_index().unwrap();

                    let src = 'inner: loop {
                        {
                            for mut stack in (thread_id..(shared_stacks.len() + thread_id))
                                .map(|id| shared_stacks[id % shared_stacks.len()].lock().unwrap())
                            {
                                if let Some(src) = stack.pop() {
                                    break 'inner src;
                                }
                            }

                            if completed.load(Ordering::Relaxed) {
                                break 'outer;
                            }
                        }
                    };

                    self.iter_node_neighbours_ids(src).for_each(|dst| {
                        if components[dst as usize].swap(
                            components[src as usize].load(Ordering::Relaxed),
                            Ordering::SeqCst,
                        ) == NOT_PRESENT
                        {
                            active_nodes_number.fetch_add(1, Ordering::SeqCst);
                            current_component_size.fetch_add(1, Ordering::SeqCst);
                            shared_stacks[rand_u64(dst as u64) as usize % shared_stacks.len()]
                                .lock()
                                .unwrap()
                                .push(dst);
                        }
                    });
                    active_nodes_number.fetch_sub(1, Ordering::SeqCst);
                });
            });
        });

        Ok((
            unsafe { std::mem::transmute::<Vec<AtomicU32>, Vec<u32>>(components) },
            components_number,
            min_component_size,
            max_component_size,
        ))
    }
}

use std::cell::UnsafeCell;

struct ThreadSafe<T> {
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for ThreadSafe<T> {}

use super::*;
use indicatif::ProgressIterator;
use std::{fs::File, io::prelude::*, io::BufWriter};

/// Structure that saves the common parameters for reading csv files.
///
/// # Attributes
/// * path: String - The path where to save the file. E.g. "/tmp/test.csv"
/// * verbose: bool - If the progress bars and logging must be displayed.
/// * separator: String - The separator to use, usually, this is "\t" for tsv and "," for csv.
/// * header: bool - If the file (will / must) have the header with the titles of the columns.
pub struct CSVFileWriter {
    pub(crate) path: String,
    pub(crate) verbose: bool,
    pub(crate) separator: String,
    pub(crate) header: bool,
}

/// # Builder methods
impl CSVFileWriter {
    /// Return new CSVFileWriter object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> CSVFileWriter {
        CSVFileWriter {
            path: path.into(),
            verbose: true,
            separator: "\t".to_string(),
            header: true,
        }
    }

    /// Write given rows iterator to file.
    ///
    /// # Arguments
    ///
    /// * `lines_number`: u64 - Number of lines to expect to write out.
    /// * `header`: Vec<String> - The header to write out, if so required.
    /// * `values`: impl Iterator<Item = Vec<String>> - Iterator of rows to write out.
    pub(crate) fn write_lines(
        &self,
        lines_number: usize,
        header: Vec<String>,
        values: impl Iterator<Item = Vec<String>>,
    ) -> Result<(), String> {
        let pb = get_loading_bar(self.verbose, "Writing to file", lines_number);

        let file = match File::create(self.path.clone()) {
            Ok(f) => Ok(f),
            Err(_) => Err(format!("Cannot open in writing the file {}", self.path)),
        }?;

        let mut stream = BufWriter::new(file);

        if self.header {
            let mut line = header.join(&self.separator);
            line.push('\n');
            match stream.write(line.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => {
                    Err("Cannot write the header. There might have been an I/O error.".to_string())
                }
            }?;
        }

        for (i, value) in values.progress_with(pb).enumerate() {
            let mut line = value.join(&self.separator);
            line.push('\n');
            match stream.write(line.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(format!(
                    "Cannot write the {i} line. There might have been an I/O error.",
                    i = i
                )),
            }?;
        }

        match stream.flush() {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to close file. There might have been an I/O error.".to_string()),
        }
    }
}

/// Return formatted vector of rows.
///
/// # Arguments
///
/// * `number_of_columns`: usize - Total number of columns to renderize.
/// * `pairs`: Vec<(String, usize)> - Vector of tuples of column names and their position.
pub(crate) fn compose_lines(number_of_columns: usize, pairs: Vec<(String, usize)>) -> Vec<String> {
    let mut values = vec!["".to_string(); number_of_columns];
    for (name, pos) in pairs {
        values[pos] = name
    }
    values
}

//! A graph representation optimized for executing random walks on huge graphs.
use super::*;
use bitvec::prelude::*;
use elias_fano_rust::EliasFano;
use rayon::prelude::*;
use roaring::RoaringBitmap;
use std::collections::HashMap;

/// A graph representation optimized for executing random walks on huge graphs.
///
/// This class should be initialized using the two constructors:
/// `graph::Graph::new_directed` or `graph::Graph::new_undirected`
///
/// # Examples
///
#[derive(Clone, Debug)]
pub struct Graph {
    /// The main datastructure where all the edges are saved
    /// in the endoced form ((src << self.node_bits) | dst) this allows us to do almost every
    /// operation in O(1) without decompressing the data.
    pub(crate) edges: EliasFano,
    /// How many bits are needed to save a node.
    pub(crate) node_bits: u8,
    /// The mask used to extract the dst value form an encoded edge.
    /// This is saved for speed sake. It's equivalent to (1 << self.node_bits) - 1;
    pub(crate) node_bit_mask: u64,

    /// Optional vector of the weights of every edge.
    /// `weights[10]` return the weight of the edge with edge_id 10
    pub(crate) weights: Option<Vec<WeightT>>,
    /// Vocabulary that save the mappings from string to index of every node type
    pub(crate) node_types: Option<NodeTypeVocabulary>,
    // This is the next attribute that will be embedded inside of edges once
    // the first refactoring is done
    /// Vocabulary that save the mappings from string to index of every edge type
    pub(crate) edge_types: Option<EdgeTypeVocabulary>,
    /// Vocabulary that save the mappings from string to index of every node
    pub(crate) nodes: Vocabulary<NodeT>,

    ////////////////////////////////////////////////////////////////////////////
    /// Cached properties
    ////////////////////////////////////////////////////////////////////////////
    /// if the graph is directed or undirected
    pub(crate) directed: bool,
    /// Number of nodes that have at least a self-loop.
    /// This means that if a nodes has multiples self-loops they will be count as one.
    pub(crate) unique_self_loop_number: NodeT,
    /// Number of self-loop edges. This counts multiple times eventual multi-graph self-loops.
    pub(crate) self_loop_number: EdgeT,
    /// Number of nodes that have at least an edge inbound or outbound.
    pub(crate) not_singleton_nodes_number: NodeT,
    /// Number of singleton nodes that have a self-loop
    pub(crate) singleton_nodes_with_self_loops_number: NodeT,
    /// How many unique edges the graph has (excluding the multi-graph ones)
    pub(crate) unique_edges_number: EdgeT,
    /// Graph name
    pub(crate) name: String,
    pub(crate) not_singleton_nodes: Option<BitVec<Lsb0, u8>>,
    pub(crate) singleton_nodes_with_self_loops: Option<RoaringBitmap>,
    pub(crate) unique_sources: Option<EliasFano>,

    /// Cache of the textual report. This is needed because in some of the bindings
    /// (such as whitin jupyter) the textual report is called multiple times like\
    /// every time the IDE tries to auto-complete.
    /// This cache must be invalidated everytime the graph is modified.
    pub(crate) cached_report: ClonableRwLock<Option<String>>,

    ////////////////////////////////////////////////////////////////////////////
    /// Elias-Fano Caching related attributes
    ////////////////////////////////////////////////////////////////////////////

    /// Vector of destinations to execute fast walks if required.
    pub(crate) destinations: Option<Vec<NodeT>>,
    /// Vector of sources to execute fast link prediction sequences if required.
    pub(crate) sources: Option<Vec<NodeT>>,
    /// Vector of outbounds to execute fast walks if required.
    pub(crate) outbounds: Option<Vec<EdgeT>>,
    // Hashmap of cached destinations to execute faster walks if required.
    pub(crate) cached_destinations: Option<HashMap<NodeT, Vec<NodeT>>>,
}

/// # Graph utility methods
impl Graph {
    pub(crate) fn new<S: Into<String>>(
        directed: bool,
        unique_self_loop_number: NodeT,
        self_loop_number: EdgeT,
        not_singleton_nodes_number: NodeT,
        singleton_nodes_with_self_loops_number: NodeT,
        unique_edges_number: EdgeT,
        edges: EliasFano,
        unique_sources: Option<EliasFano>,
        nodes: Vocabulary<NodeT>,
        node_bit_mask: EdgeT,
        node_bits: u8,
        edge_types: Option<EdgeTypeVocabulary>,
        name: S,
        weights: Option<Vec<WeightT>>,
        node_types: Option<NodeTypeVocabulary>,
        not_singleton_nodes: Option<BitVec<Lsb0, u8>>,
        singleton_nodes_with_self_loops: Option<RoaringBitmap>
    ) -> Graph {
        Graph {
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            node_bit_mask,
            node_bits,
            weights,
            node_types: node_types.map(|nts| nts.set_numeric_ids(false)),
            edge_types: edge_types.map(|ets| ets.set_numeric_ids(false)),
            nodes: nodes.set_numeric_ids(false),
            sources: None,
            destinations: None,
            outbounds: None,
            cached_destinations: None,
            name: name.into(),
            not_singleton_nodes,
            singleton_nodes_with_self_loops,
            cached_report: ClonableRwLock::new(None),
        }
    }

    /// Return true if given graph has any edge overlapping with current graph.
    ///
    /// # Arguments
    ///
    /// * other: Graph - The graph to check against.
    ///
    pub fn overlaps(&self, other: &Graph) -> Result<bool, String> {
        Ok(match self.is_compatible(other)? {
            true => other
                .par_iter_edge_with_type_ids(other.directed)
                .any(|(_, src, dst, et)| self.has_edge_with_type_by_node_ids(src, dst, et)),
            false => other.par_iter_edge_with_type(other.directed).any(
                |(_, _, src_name, _, dst_name, _, edge_type_name)| {
                    self.has_edge_with_type_by_node_names(
                        &src_name,
                        &dst_name,
                        edge_type_name.as_ref(),
                    )
                },
            ),
        })
    }

    /// Return true if given graph edges are all contained within current graph.
    ///
    /// # Arguments
    ///
    /// * other: Graph - The graph to check against.
    ///
    pub fn contains(&self, other: &Graph) -> Result<bool, String> {
        Ok(match self.is_compatible(other)? {
            true => other
                .par_iter_edge_with_type_ids(other.directed)
                .all(|(_, src, dst, et)| self.has_edge_with_type_by_node_ids(src, dst, et)),
            false => other.par_iter_edge_with_type(other.directed).all(
                |(_, _, src_name, _, dst_name, _, edge_type_name)| {
                    self.has_edge_with_type_by_node_names(
                        &src_name,
                        &dst_name,
                        edge_type_name.as_ref(),
                    )
                },
            ),
        })
    }
}

use super::*;
use indicatif::ProgressIterator;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;
use std::collections::HashMap;
use vec_rand::gen_random_vec;
use vec_rand::xorshift::xorshift;

#[inline(always)]
/// Computes val % n using lemires fast method for u32.
/// https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
/// This is supposed to be ~5 times faster.
fn fast_u32_modulo(val: u32, n: u32) -> u32 {
    ((val as u64 * n as u64) >> 32) as u32
}

/// Return training batches for Word2Vec models.
///
/// The batch is composed of a tuple as the following:
///
/// - (Contexts indices, central nodes indices): the tuple of nodes
///
/// This does not provide any output value as the model uses NCE loss
/// and basically the central nodes that are fed as inputs work as the
/// outputs value.
///
/// # Arguments
///
/// * sequences: Vec<Vec<usize>> - the sequence of sequences of integers to preprocess.
/// * window_size: usize - Window size to consider for the sequences.
///
pub fn word2vec<'a>(
    sequences: impl ParallelIterator<Item = Vec<NodeT>> + 'a,
    window_size: usize,
) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a, String> {
    Ok(sequences.flat_map_iter(move |sequence| {
        let sequence_length = sequence.len();
        if sequence_length < window_size * 2 + 1 {
            panic!(
                "
            Cannot compute word2vec, got a sequence of length {} and window size {}.
            for the current window_size the minimum sequence length required is {}",
                sequence_length,
                window_size,
                window_size * 2 + 1,
            );
        }
        (window_size..(sequence_length - window_size)).map(move |i| {
            (
                (i - window_size..i)
                    .chain(i + 1..window_size + i + 1)
                    .map(|j| sequence[j])
                    .collect(),
                sequence[i],
            )
        })
    }))
}

/// Return triple with CSR representation of cooccurrence matrix.
///
/// The first vector has the sources, the second vector the destinations
/// and the third one contains the min-max normalized frequencies.
///
/// # Arguments
///
/// * sequences:Vec<Vec<usize>> - the sequence of sequences of integers to preprocess.
/// * window_size: Option<usize> - Window size to consider for the sequences.
/// * verbose: Option<bool>,
///     whether to show the progress bars.
///     The default behaviour is false.
///     
pub fn cooccurence_matrix(
    sequences: impl ParallelIterator<Item = Vec<NodeT>>,
    window_size: usize,
    number_of_sequences: usize,
    verbose: bool,
) -> Result<(usize, impl Iterator<Item=(NodeT, NodeT, f64)>), String> {
    let mut cooccurence_matrix: HashMap<(NodeT, NodeT), f64> = HashMap::new();
    let mut max_frequency = 0.0;
    let pb1 = get_loading_bar(verbose, "Computing frequencies", number_of_sequences);

    // TODO!: Avoid this collect and create the cooccurrence matrix in a parallel way.
    // We are currently working on this but is terribly non-trivial,
    // as most parallel implementations end up being slower than sequential
    // ones or require massive amounts of additional memory.
    let vec = sequences.collect::<Vec<Vec<NodeT>>>();
    vec.iter().progress_with(pb1).for_each(|sequence| {
        let walk_length = sequence.len();
        for (central_index, &central_word_id) in sequence.iter().enumerate() {
            let upperbound = std::cmp::min(1 + window_size, walk_length - central_index);

            for distance in 1..upperbound {
                let context_id = sequence[central_index + distance];

                let (smaller, bigger) = (
                    std::cmp::min(central_word_id, context_id),
                    std::cmp::max(central_word_id, context_id),
                );

                let freq = 1.0 / distance as f64;

                // Get the current value for this pair of nodes
                let ptr = cooccurence_matrix
                    .entry((smaller, bigger))
                    .and_modify(|e| *e += freq)
                    .or_insert(freq);
                // Update the max
                if *ptr > max_frequency {
                    max_frequency = *ptr;
                }
            }
        }
    });

    let number_of_elements = cooccurence_matrix.len();
    let pb2 = get_loading_bar(
        verbose,
        "Converting mapping into CSR matrix",
        cooccurence_matrix.len(),
    );
    Ok((
        number_of_elements,
        cooccurence_matrix
            .into_iter()
            .progress_with(pb2)
            .map(move |((word, context), frequency)| {
                (word, context, frequency / max_frequency)
            })
    ))
}

/// # Preprocessing for ML algorithms on graph.
impl Graph {
    /// Return training batches for Node2Vec models.
    ///
    /// The batch is composed of a tuple as the following:
    ///
    /// - (Contexts indices, central nodes indices): the tuple of nodes
    ///
    /// This does not provide any output value as the model uses NCE loss
    /// and basically the central nodes that are fed as inputs work as the
    /// outputs value.
    ///
    /// # Arguments
    ///
    /// * walk_parameters: &WalksParameters - the weighted walks parameters.
    /// * quantity: usize - Number of nodes to consider.
    /// * window_size: usize - Window size to consider for the sequences.
    ///
    pub fn node2vec<'a>(
        &'a self,
        walk_parameters: &'a WalksParameters,
        quantity: NodeT,
        window_size: usize,
    ) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, NodeT)> + 'a, String> {
        // do the walks and check the result
        word2vec(
            self.random_walks_iter(quantity, walk_parameters)?,
            window_size,
        )
    }

    /// Return triple with CSR representation of cooccurrence matrix.
    ///
    /// The first vector has the sources, the second vector the destinations
    /// and the third one contains the min-max normalized frequencies.
    ///
    /// # Arguments
    ///
    /// * parameters: &WalksParameters - the walks parameters.
    /// * window_size: Option<usize> - Window size to consider for the sequences.
    /// * verbose: Option<bool>,
    ///     whether to show the progress bars.
    ///     The default behaviour is false.
    ///     
    pub fn cooccurence_matrix<'a>(
        &'a self,
        walks_parameters: &'a WalksParameters,
        window_size: usize,
        verbose: bool,
    ) -> Result<(usize, impl Iterator<Item=(NodeT, NodeT, f64)> + 'a), String> {
        if !self.has_edges() {
            return Err(
                "The cooccurence matrix on a graph without edges is not defined.".to_string(),
            );
        }
        let walks = self.complete_walks_iter(walks_parameters)?;
        cooccurence_matrix(
            walks,
            window_size,
            (self.get_unique_source_nodes_number() * walks_parameters.iterations) as usize,
            verbose,
        )
    }

    /// Return iterator over neighbours for the given node ID, optionally including given node ID.
    ///
    /// This method is meant to be used to predict node labels using the NoLaN model.
    ///
    /// If you need to predict the node label of a node, not during training,
    /// use `max_neighbours=None`.
    ///
    /// # Arguments
    ///
    /// * `central_node_id`: NodeT - The node ID to retrieve neighbours for.
    /// * `random_state`: u64 - The random state to use to extract the neighbours.
    /// * `include_central_node`: bool - Whether to include the node ID in the returned iterator.
    /// * `offset`: NodeT - Offset for padding porposes.
    /// * `max_neighbours`: &Option<NodeT> - Number of maximum neighbours to consider.
    ///
    pub(crate) fn get_neighbours_by_node_id(
        &self,
        central_node_id: NodeT,
        random_state: u64,
        include_central_node: bool,
        offset: NodeT,
        max_neighbours: Option<NodeT>,
    ) -> impl Iterator<Item = NodeT> + '_ {
        (if include_central_node {
            vec![central_node_id]
        } else {
            vec![]
        })
        .into_iter()
        .chain(
            self.get_unchecked_node_destinations_by_node_id(
                central_node_id,
                random_state,
                max_neighbours,
            )
            .into_iter(),
        )
        .map(move |node_id| node_id + offset)
    }

    /// Return tuple with iterator over neighbours for the given node ID, optionally including given node ID, and node type.
    ///
    /// This method is meant to be used to predict node labels using the NoLaN model.
    ///
    /// If you need to predict the node label of a node, not during training,
    /// use `max_neighbours=None`.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - The node ID to retrieve neighbours for.
    /// * `random_state`: u64 - The random state to use to extract the neighbours.
    /// * `include_central_node`: bool - Whether to include the node ID in the returned iterator.
    /// * `offset`: NodeT - Offset for padding porposes.
    /// * `max_neighbours`: &Option<NodeT> - Number of maximum neighbours to consider.
    ///
    pub(crate) fn get_node_label_prediction_tuple_by_node_id(
        &self,
        node_id: NodeT,
        random_state: u64,
        include_central_node: bool,
        offset: NodeT,
        max_neighbours: Option<NodeT>,
    ) -> (impl Iterator<Item = NodeT> + '_, Option<Vec<NodeTypeT>>) {
        (
            self.get_neighbours_by_node_id(
                node_id,
                random_state,
                include_central_node,
                offset,
                max_neighbours,
            ),
            self.get_unchecked_node_type_id_by_node_id(node_id),
        )
    }

    /// Return iterator over neighbours for the given node IDs, optionally including given the node IDs, and node type.
    ///
    /// This method is meant to be used to predict node labels using the NoLaN model.
    ///
    /// If you need to predict the node label of a node, not during training,
    /// use `max_neighbours=None`.
    ///
    /// # Arguments
    ///
    /// * `node_ids`: Vec<NodeT> - The node ID to retrieve neighbours for.
    /// * `random_state`: u64 - The random state to use to extract the neighbours.
    /// * `include_central_node`: bool - Whether to include the node ID in the returned iterator.
    /// * `offset`: NodeT - Offset for padding porposes.
    /// * `max_neighbours`: &Option<NodeT> - Number of maximum neighbours to consider.
    ///
    /// # Examples
    /// Suppose you want to the get the neighbours of the first 10 nodes:
    /// ```rust
    /// # use rayon::iter::ParallelIterator;
    /// # use graph::NodeT;
    /// # use rayon::iter::IndexedParallelIterator;
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, false, false, false).unwrap();
    /// let node_ids = (0..10).collect::<Vec<NodeT>>();
    /// let include_central_nodes = true;
    /// let offset = 0;
    /// let max_neighbours = 5;
    /// let iterator = graph.get_node_label_prediction_tuple_by_node_ids(
    ///    node_ids.clone(), 42, include_central_nodes, offset, Some(max_neighbours)
    /// ).unwrap();
    /// iterator.enumerate().for_each(|(i, (neighbours_iter, labels))|{
    ///     for (j, node_id) in neighbours_iter.enumerate(){
    ///         if j==0 && include_central_nodes{
    ///             assert!(node_id==node_ids[i]);
    ///         }
    ///         assert!(
    ///             max_neighbours + include_central_nodes as NodeT > j as NodeT,
    ///             "The index {} is higher than the given maximum neighbours number {}!",
    ///             j,
    ///             max_neighbours
    ///         );
    ///     }
    /// });
    /// ```
    ///
    pub fn get_node_label_prediction_tuple_by_node_ids(
        &self,
        node_ids: Vec<NodeT>,
        random_state: u64,
        include_central_node: bool,
        offset: NodeT,
        max_neighbours: Option<NodeT>,
    ) -> Result<
        impl Iterator<Item = (impl Iterator<Item = NodeT> + '_, Option<Vec<NodeTypeT>>)> + '_,
        String,
    > {
        if !self.has_node_types() {
            return Err("The current graph instance does not have node types!".to_string());
        }
        Ok(node_ids.into_iter().map(move |node_id| {
            self.get_node_label_prediction_tuple_by_node_id(
                node_id,
                random_state,
                include_central_node,
                offset,
                max_neighbours,
            )
        }))
    }

    /// Returns triple with the degrees of source nodes, destination nodes and labels for training model for link prediction.
    /// This method is just for setting the lowerbound on the simplest possible model.
    ///
    /// # Arguments
    ///
    /// * idx:u64 - The index of the batch to generate, behaves like a random random_state,
    /// * batch_size: usize - The maximal size of the batch to generate,
    /// * normalize: bool - Divide the degrees by the max, this way the values are in [0, 1],
    /// * negative_samples: f64 - The component of netagetive samples to use,
    /// * avoid_false_negatives: bool - whether to remove the false negatives when generated.
    ///     - It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * maximal_sampling_attempts: usize - Number of attempts to execute to sample the negative edges.
    /// * graph_to_avoid: Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    pub fn link_prediction_degrees<'a>(
        &'a self,
        idx: u64,
        batch_size: usize,
        normalize: bool,
        negative_samples: f64,
        avoid_false_negatives: bool,
        maximal_sampling_attempts: usize,
        graph_to_avoid: &'a Option<&Graph>,
    ) -> Result<impl ParallelIterator<Item = (usize, f64, f64, bool)> + 'a, String> {
        let iter = self.link_prediction_ids(
            idx,
            batch_size,
            negative_samples,
            avoid_false_negatives,
            maximal_sampling_attempts,
            graph_to_avoid,
        )?;

        let max_degree = match normalize {
            true => self.get_max_node_degree()? as f64,
            false => 1.0,
        };

        Ok(iter.map(move |(index, src, dst, label)| {
            (
                index,
                self.get_node_degree_by_node_id(src).unwrap() as f64 / max_degree,
                self.get_node_degree_by_node_id(dst).unwrap() as f64 / max_degree,
                label,
            )
        }))
    }

    /// Returns triple with the ids of source nodes, destination nodes and labels for training model for link prediction.
    ///
    /// # Arguments
    ///
    /// * idx:u64 - The index of the batch to generate, behaves like a random random_state,
    /// * batch_size: usize - The maximal size of the batch to generate,
    /// * negative_samples: f64 - The component of netagetive samples to use,
    /// * avoid_false_negatives: bool - whether to remove the false negatives when generated.
    ///     - It should be left to false, as it has very limited impact on the training, but enabling this will slow things down.
    /// * maximal_sampling_attempts: usize - Number of attempts to execute to sample the negative edges.
    /// * graph_to_avoid: Option<&Graph> - The graph whose edges are to be avoided during the generation of false negatives,
    ///
    pub fn link_prediction_ids<'a>(
        &'a self,
        idx: u64,
        batch_size: usize,
        negative_samples: f64,
        avoid_false_negatives: bool,
        maximal_sampling_attempts: usize,
        graph_to_avoid: &'a Option<&Graph>,
    ) -> Result<impl ParallelIterator<Item = (usize, NodeT, NodeT, bool)> + 'a, String> {
        // xor the random_state with a constant so that we have a good amount of 0s and 1s in the number
        // even with low values (this is needed becasue the random_state 0 make xorshift return always 0)
        let random_state = idx ^ SEED_XOR as u64;

        if negative_samples < 0.0 || !negative_samples.is_finite() {
            return Err("Negative sample must be a posive real value.".to_string());
        }

        // The number of negatives is given by computing their fraction of batchsize
        let negative_number: usize =
            ((batch_size as f64 / (1.0 + negative_samples)) * negative_samples) as usize;
        // All the remaining values then are positives
        let positive_number: usize = batch_size - negative_number;
        let graph_has_no_self_loops = !self.has_selfloops();

        let edges_number = self.get_directed_edges_number() as u64;
        let nodes_number = self.get_nodes_number() as u32;

        let mut rng: StdRng = SeedableRng::seed_from_u64(random_state);
        let random_values = gen_random_vec(batch_size, random_state);
        let mut indices: Vec<usize> = (0..batch_size).collect();
        indices.shuffle(&mut rng);

        Ok((0..batch_size)
            .into_par_iter()
            .map(move |i| {
                let mut sampled = random_values[i];
                if i < positive_number{
                    let (src, dst) = self.get_node_ids_from_edge_id(sampled % edges_number);
                    (indices[i], src, dst, true)
                } else {
                    for _ in 0..maximal_sampling_attempts {
                        // split the random u64 into 2 u32 and mod them to have
                        // usable nodes (this is slightly biased towards low values)
                        let src = fast_u32_modulo((sampled & 0xffffffff) as u32, nodes_number);
                        let dst = fast_u32_modulo((sampled >> 32) as u32, nodes_number);

                        if avoid_false_negatives && self.has_edge_by_node_ids(src, dst) {
                            sampled = xorshift(sampled);
                            continue;
                        }

                        if let Some(g) = &graph_to_avoid {
                            if g.has_edge_by_node_ids(src, dst)  {
                                sampled = xorshift(sampled);
                                continue;
                            }
                        }

                        if graph_has_no_self_loops && src == dst {
                            sampled = xorshift(sampled);
                            continue;
                        }

                        return (indices[i], src, dst, false);
                    }
                    panic!(
                        concat!(
                            "Executed more than {} attempts to sample a negative edge.\n",
                            "If your graph is so small that you see this error, you may want to consider ",
                            "using one of the edge embedding transformer from the Embiggen library."
                        ),
                        maximal_sampling_attempts
                    );
                }
            }))
    }
}

use super::*;
use indicatif::{ProgressBar, ProgressStyle};

pub(crate) fn get_loading_bar(verbose: bool, desc: &str, total_iterations: usize) -> ProgressBar {
    if verbose {
        let pb = ProgressBar::new(total_iterations as u64);
        pb.set_draw_delta(total_iterations as u64 / 100);
        pb.set_style(ProgressStyle::default_bar().template(&format!(
            "{desc} {{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})",
            desc=desc
        )));
        pb
    } else {
        ProgressBar::hidden()
    }
}

/// Return true if the given weight is near to one.
pub(crate) fn not_one(weight: WeightT) -> bool {
    (weight - 1.0).abs() > WeightT::EPSILON
}

impl Graph {
    /// Return vector of edges to be inserted in the holdout.
    pub(crate) fn compute_edge_ids_vector(
        &self,
        edge_id: EdgeT,
        src: NodeT,
        dst: NodeT,
        include_all_edge_types: bool,
    ) -> Vec<EdgeT> {
        if include_all_edge_types {
            let (min_edge_id, max_edge_id) =
                self.get_unchecked_minmax_edge_ids_by_node_ids(src, dst);
            (min_edge_id..max_edge_id).collect::<Vec<EdgeT>>()
        } else {
            vec![edge_id]
        }
    }
}

/// Return validated weight.
///
/// A weight, to be valid in the context of graph machine learning
/// as we have defined, must be strictly positive and non infinite.
///
/// # Arguments
///
/// * weight: WeightT - The weight to validate.
///
/// # Examples
/// The weight can be validated as follows:
/// ```rust
/// # use graph::utils::validate_weight;
/// assert!(validate_weight(0.0).is_err());
/// assert!(validate_weight(-1.0).is_err());
/// assert!(validate_weight(2.0).is_ok());
/// assert_eq!(validate_weight(2.0).unwrap(), 2.0);
/// ```
///
pub fn validate_weight(weight: WeightT) -> Result<WeightT, String> {
    if weight.is_finite() && weight > 0.0 {
        Ok(weight)
    } else {
        Err(format!(
            "The weight is '{}' but the weights must be strictly positives and finite.",
            weight
        ))
    }
}

/// Return given weight parsed from string to float.
///
/// # Arguments
///
/// * weight: String - The weight to be parsed.
///
/// # Examples
/// The weight can be validated as follows:
/// ```rust
/// # use graph::utils::parse_weight;
/// assert!(parse_weight("0.0".to_string()).is_ok());
/// assert!(parse_weight("-1.0".to_string()).is_ok());
/// assert!(parse_weight("2.0".to_string()).is_ok());
/// assert!(parse_weight("2ghgjh.0".to_string()).is_err());
/// assert_eq!(parse_weight("2.0".to_string()).unwrap(), 2.0);
/// ```
///
pub fn parse_weight(weight: String) -> Result<WeightT, String> {
    match weight.parse::<WeightT>() {
        Ok(val) => Ok(val),
        Err(_) => Err(format!("Cannot parse weight {} as a float.", weight)),
    }
}

use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs::File, io::prelude::*, io::BufReader};

use crate::utils::get_loading_bar;

/// Structure that saves the common parameters for reading csv files.
///
/// # Attributes
///
/// * `path`: String - The of the file to read. E.g. "/tmp/test.csv"
/// * `verbose`: bool - If the progress bars and logging must be displayed.
/// * `separator`: String - The separator to use, usually, this is "\t" for tsv and "," for csv.
/// * `header`: bool - If the file (will / must) have the header with the titles of the columns.
/// * `rows_to_skip`: usize - When reading, how many lines to skip before starting to read the file.
/// * `ignore_duplicates`: bool - Whether the program should raise an exception or not when the file contains duplicated edges / nodes.
/// * `csv_is_correct`: bool - Pinky promise that the file is well made.
/// * `max_rows_number`: Option<u64> -if the program should stop reading after a certain number of rows.
/// * `list_name`: String - The name of the list that is being loaded.
/// * `graph_name`: String - The name of graph that is being loaded.
///
#[derive(Clone)]
pub struct CSVFileReader {
    pub(crate) path: String,
    pub(crate) verbose: bool,
    pub(crate) separator: String,
    pub(crate) header: bool,
    pub(crate) rows_to_skip: usize,
    pub(crate) ignore_duplicates: bool,
    pub(crate) csv_is_correct: bool,
    pub(crate) max_rows_number: Option<u64>,
    pub(crate) comment_symbol: Option<String>,
    pub(crate) list_name: String,
    pub(crate) graph_name: String,
}

/// # Builder methods
impl CSVFileReader {
    /// Return new CSVFileReader object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    /// * list_name: String - Name of the list that is being loaded.
    ///
    pub fn new<S: Into<String>>(path: S, list_name: String) -> Result<CSVFileReader, String> {
        let path = path.into();
        // check file existance
        match File::open(&path) {
            Ok(_) => Ok(CSVFileReader {
                path,
                verbose: true,
                separator: "\t".to_string(),
                header: true,
                rows_to_skip: 0,
                ignore_duplicates: true,
                csv_is_correct: false,
                max_rows_number: None,
                comment_symbol: None,
                list_name,
                graph_name: "Graph".to_string(),
            }),
            Err(_) => Err(format!("Cannot open the file at {}", path)),
        }
    }

    /// Read the whole file and return how many rows it has.
    pub(crate) fn count_rows(&self) -> usize {
        std::cmp::min(
            BufReader::new(File::open(&self.path).unwrap())
                .lines()
                .count(),
            self.max_rows_number.unwrap_or(u64::MAX) as usize,
        )
    }

    /// Return list of components of the header.
    pub fn get_header(&self) -> Result<Vec<String>, String> {
        if let Some(first_line) = self.get_lines_iterator(false)?.next() {
            Ok(first_line?
                .split(&self.separator)
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
        } else {
            Err("The given file has no lines!".to_string())
        }
    }

    pub fn get_lines_iterator(
        &self,
        skip_header: bool,
    ) -> Result<impl Iterator<Item = Result<String, String>> + '_, String> {
        let rows_to_skip = match skip_header {
            true => match (self.rows_to_skip as u64).checked_add(self.header as u64) {
                Some(v) => Ok(v),
                None => Err(concat!(
                    "This overflow was caused because rows to skip = 2**64 - 1",
                    "and header is setted to true which causes to skip one extra line.",
                    "Do you **really** want to skip 18446744073709551615 lines? Bad person. Bad."
                )),
            }?,
            false => self.rows_to_skip as u64,
        } as usize;
        Ok(BufReader::new(File::open(&self.path).unwrap())
            .lines()
            .map(|line| match line {
                Ok(l)=>Ok(l),
                Err(_)=>Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string()),
            })
            .filter_ok(move |line| !line.is_empty() && match &self.comment_symbol {
                Some(cs) => !line.starts_with(cs),
                _ => true,
            })
            .skip(rows_to_skip))
    }

    /// Return elements of the first line not to be skipped.
    pub fn get_elements_per_line(&self) -> Result<usize, String> {
        let first_line = self.get_lines_iterator(true)?.next();
        match first_line {
            Some(fl) => {
                match fl {
                    Ok(f) => {
                        Ok(f.matches(&self.separator).count() + 1)
                    },
                    Err(_) => Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string())
                }
            },
            None => Err(concat!(
                "Unable to read the first non skipped line of the file.\n",
                "The file has possibly less than the expected amount of lines"
            ).to_string())
        }
    }

    /// Return iterator that read a CSV file rows.
    pub(crate) fn read_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<Vec<Option<String>>, String>> + '_, String> {
        let pb = get_loading_bar(
            self.verbose,
            format!("Reading {}'s {}", self.graph_name, self.list_name).as_ref(),
            if self.verbose { self.count_rows() } else { 0 },
        );

        let number_of_elements_per_line = self.get_elements_per_line()?;
        Ok(self
            .get_lines_iterator(true)?
            .progress_with(pb)
            // skip empty lines
            .take(self.max_rows_number.unwrap_or(u64::MAX) as usize)
            // Handling NaN values and padding them to the number of rows
            .map_ok(move |line| {
                let mut elements: Vec<Option<String>> = line
                    .split(&self.separator)
                    .map(|element| match element.is_empty() {
                        true => None,
                        false => Some(element.to_string()),
                    })
                    .collect();
                elements.resize(number_of_elements_per_line, None);
                elements
            }))
    }

    /// Return number of the given column in header.
    ///
    /// # Arguments
    ///
    /// * column_name: String - Column to get the number of.
    ///
    pub fn get_column_number(&self, column_name: String) -> Result<usize, String> {
        let header = self.get_header()?;

        match header.iter().position(|x| *x == column_name) {
            Some(column_number) => Ok(column_number),
            None => Err(format!(
                "The column '{}' is not present in the header\n{:?}",
                column_name, header
            )),
        }
    }
}

use super::*;
use indicatif::ProgressIterator;

impl Graph {
    /// Return whether nodes are remappable to those of the given graph.
    ///
    /// # Arguments
    /// other: &Graph - graph towards remap the nodes to.
    ///
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(graph.are_nodes_remappable(&graph));
    /// ```
    /// Two different graphs, like Cora and STRING, are not remappable:
    /// ```rust
    /// # let cora = graph::test_utilities::load_cora().unwrap();
    /// # let ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(!cora.are_nodes_remappable(&ppi));
    /// ```
    ///
    pub fn are_nodes_remappable(&self, other: &Graph) -> bool {
        if self.get_nodes_number() != other.get_nodes_number() {
            return false;
        }
        self.iter_nodes().all(|(_, node_name, _, node_type)| {
            other.has_node_with_type_by_node_name(&node_name, node_type)
        })
    }

    /// Return graph remapped towards nodes of the given graph.
    ///
    /// # Arguments
    ///
    /// * other: &Graph - The graph to remap towards.
    /// * verbose: bool - whether to show a loding bar.
    ///
    /// # Example
    /// A graph is always remappable to itself:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert_eq!(graph, graph.remap(&graph, false).unwrap());
    /// ```
    ///
    pub fn remap(&self, other: &Graph, verbose: bool) -> Result<Graph, String> {
        let pb = get_loading_bar(
            verbose,
            format!("Building remapped {}", self.name).as_ref(),
            self.get_directed_edges_number() as usize,
        );

        if !self.are_nodes_remappable(&other) {
            return Err("The two graphs nodes sets are not remappable one-another.".to_owned());
        }

        Graph::from_integer_unsorted(
            self.iter_edge_with_type_and_weight(true)
                .progress_with(pb)
                .map(|(_, _, src_name, _, dst_name, _, edge_type, weight)| {
                    Ok((
                        other.get_unchecked_node_id_by_node_name(&src_name),
                        other.get_unchecked_node_id_by_node_name(&dst_name),
                        edge_type.and_then(|et| {
                            self.get_unchecked_edge_type_id_by_edge_type_name(et.as_str())
                        }),
                        weight,
                    ))
                }),
            other.nodes.clone(),
            other.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.directed,
            self.name.clone(),
            false,
            self.has_edge_types(),
            self.has_weights(),
            verbose,
            self.has_singletons(),
            self.has_singleton_nodes_with_self_loops(),
            self.has_trap_nodes(),
        )
    }
}

use itertools::Itertools;

use super::*;
/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
#[derive(Clone)]
pub struct EdgeFileReader {
    pub(crate) reader: CSVFileReader,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column_number: Option<usize>,
    pub(crate) default_edge_type: Option<String>,
    pub(crate) weights_column_number: Option<usize>,
    pub(crate) default_weight: Option<WeightT>,
    pub(crate) skip_self_loops: bool,
    pub(crate) numeric_edge_type_ids: bool,
    pub(crate) numeric_node_ids: bool,
    pub(crate) skip_weights_if_unavailable: bool,
    pub(crate) skip_edge_types_if_unavailable: bool,
    pub(crate) might_have_singletons_with_selfloops: bool,
    pub(crate) might_have_trap_nodes: bool,
}

impl EdgeFileReader {
    /// Return new EdgeFileReader object.
    ///
    /// # Arguments
    ///
    /// * reader: CSVFilereader - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> Result<EdgeFileReader, String> {
        Ok(EdgeFileReader {
            reader: CSVFileReader::new(path, "edge list".to_owned())?,
            sources_column_number: 0,
            destinations_column_number: 1,
            edge_types_column_number: None,
            default_edge_type: None,
            weights_column_number: None,
            default_weight: None,
            skip_self_loops: false,
            numeric_edge_type_ids: false,
            numeric_node_ids: false,
            skip_weights_if_unavailable: false,
            skip_edge_types_if_unavailable: false,
            might_have_singletons_with_selfloops: true,
            might_have_trap_nodes: true,
        })
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column<S: Into<String>>(
        mut self,
        sources_column: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = sources_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }

            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self = self.set_sources_column_number(Some(ecn))?;
                }
                Err(e) => {
                    if !self.skip_edge_types_if_unavailable {
                        return Err(e);
                    }
                }
            }
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column_number: Option<usize> - The sources column number to use for the file.
    ///
    pub fn set_sources_column_number(
        mut self,
        sources_column_number: Option<usize>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = sources_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The source column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_elements
                ));
            }
            self.sources_column_number = column;
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column<S: Into<String>>(
        mut self,
        destinations_column: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = destinations_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }
            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self = self.set_destinations_column_number(Some(ecn))?;
                }
                Err(e) => {
                    if !self.skip_edge_types_if_unavailable {
                        return Err(e);
                    }
                }
            }
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column_number: Option<usize> - The destinations column number to use for the file.
    ///
    pub fn set_destinations_column_number(
        mut self,
        destinations_column_number: Option<usize>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = destinations_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The destinations column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_elements
                ));
            }
            self.destinations_column_number = column;
        }
        Ok(self)
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destination_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column<S: Into<String>>(
        mut self,
        edge_type_column: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = edge_type_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }
            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self = self.set_edge_types_column_number(Some(ecn))?;
                }
                Err(e) => {
                    if !self.skip_edge_types_if_unavailable {
                        return Err(e);
                    }
                }
            }
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column_number: Option<usize> - The edge_types column number to use for the file.
    ///
    pub fn set_edge_types_column_number(
        mut self,
        edge_types_column_number: Option<usize>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(etcn) = &edge_types_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if *etcn >= expected_elements {
                if !self.skip_edge_types_if_unavailable {
                    return Err(format!(
                        concat!(
                            "The edge types column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        etcn, expected_elements
                    ));
                }
            } else {
                self.edge_types_column_number = edge_types_column_number;
            }
        }
        Ok(self)
    }

    /// Set the column of the edge weights.
    ///
    /// # Arguments
    ///
    /// * weights_column: Option<String> - The edge weights column to use for the file.
    ///
    pub fn set_weights_column<S: Into<String>>(
        mut self,
        weights_column: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(column) = weights_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given edge weights column is empty.".to_owned());
            }
            match self.reader.get_column_number(column) {
                Ok(wcn) => {
                    self = self.set_weights_column_number(Some(wcn))?;
                }
                Err(e) => {
                    if !self.skip_weights_if_unavailable {
                        return Err(e);
                    }
                }
            }
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column_number: Option<usize> - The weights column number to use for the file.
    ///
    pub fn set_weights_column_number(
        mut self,
        weights_column_number: Option<usize>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(wcn) = &weights_column_number {
            let expected_elements = self.reader.get_elements_per_line()?;
            if *wcn >= expected_elements {
                if !self.skip_edge_types_if_unavailable {
                    return Err(format!(
                        concat!(
                            "The weights column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        wcn, expected_elements
                    ));
                }
            } else {
                self.weights_column_number = weights_column_number;
            }
        }
        Ok(self)
    }

    /// Set whether to automatically skip weights if they are not avaitable instead of raising an exception.
    ///
    /// # Arguments
    ///
    /// * skip_weights_if_unavailable: Option<bool> - whether to skip weights if they are not available.
    ///
    pub fn set_skip_weights_if_unavailable(
        mut self,
        skip_weights_if_unavailable: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(skip) = skip_weights_if_unavailable {
            self.skip_weights_if_unavailable = skip;
        }
        self
    }

    /// Set whether to automatically skip edge types if they are not avaitable instead of raising an exception.
    ///
    /// # Arguments
    ///
    /// * skip_edge_types_if_unavailable: Option<bool> - whether to skip edge types if they are not available.
    ///
    pub fn set_skip_edge_types_if_unavailable(
        mut self,
        skip_edge_types_if_unavailable: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(skip) = skip_edge_types_if_unavailable {
            self.skip_edge_types_if_unavailable = skip;
        }
        self
    }

    /// Set the default default_weight.
    ///
    /// # Arguments
    ///
    /// * default_weight: Option<WeightT> - The default_weight to use when default_weight is missing.
    ///
    pub fn set_default_weight(mut self, default_weight: Option<WeightT>) -> EdgeFileReader {
        self.default_weight = default_weight;
        self
    }

    /// Set the name of the graph to be loaded.
    ///
    /// # Arguments
    ///
    /// * graph_name: String - The name of the graph to be loaded.
    ///
    pub(crate) fn set_graph_name(mut self, graph_name: String) -> EdgeFileReader {
        self.reader.graph_name = graph_name;
        self
    }

    /// Set the default edge type.
    ///
    /// # Arguments
    ///
    /// * default_edge_type: Option<String> - The edge type to use when edge type is missing.
    ///
    pub fn set_default_edge_type<S: Into<String>>(
        mut self,
        default_edge_type: Option<S>,
    ) -> EdgeFileReader {
        self.default_edge_type = default_edge_type.map(|val| val.into());
        self
    }

    /// Set whether should ignore or not selfloops.
    ///
    /// # Arguments
    ///
    /// * skip_self_loops: Option<bool> - whether should ignore or not selfloops.
    ///
    pub fn set_skip_self_loops(mut self, skip_self_loops: Option<bool>) -> EdgeFileReader {
        if let Some(ssl) = skip_self_loops {
            self.skip_self_loops = ssl;
            self.might_have_singletons_with_selfloops = !ssl;
        }
        self
    }

    /// Set whether the CSV is expected to be well written.
    ///
    /// # Arguments
    ///
    /// * csv_is_correct: Option<bool> - Whether you pinky swear the edge list is correct.
    ///
    pub fn set_csv_is_correct(mut self, csv_is_correct: Option<bool>) -> EdgeFileReader {
        if let Some(cic) = csv_is_correct {
            self.reader.csv_is_correct = cic;
        }
        self
    }

    /// Set the comment symbol to use to skip the lines.
    ///
    /// # Arguments
    ///
    /// * comment_symbol: Option<String> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_comment_symbol(
        mut self,
        comment_symbol: Option<String>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(cs) = comment_symbol {
            if cs.is_empty() {
                return Err("The given comment symbol is empty.".to_string());
            }
            self.reader.comment_symbol = Some(cs);
        }
        Ok(self)
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> EdgeFileReader {
        if let Some(v) = verbose {
            self.reader.verbose = v;
        }
        self
    }

    /// Set whether you pinky promise that this graph has singletons with self-loops or not.
    ///
    /// # Arguments
    ///
    /// * might_have_singletons_with_selfloops: Option<bool> - Whether this graph has singletons with self-loops.
    ///
    pub fn set_might_have_singletons_with_selfloops(
        mut self,
        might_have_singletons_with_selfloops: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(skip) = might_have_singletons_with_selfloops {
            self.might_have_singletons_with_selfloops = !self.skip_self_loops && skip;
        }
        self
    }

    /// Set whether you pinky promise that this graph has trap nodes or not.
    ///
    /// # Arguments
    ///
    /// * might_have_trap_nodes: Option<bool> - Whether this graph has trap nodes with self-loops.
    ///
    pub fn set_might_have_trap_nodes(
        mut self,
        might_have_trap_nodes: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(skip) = might_have_trap_nodes {
            self.might_have_trap_nodes = skip;
        }
        self
    }

    ///
    /// * numeric_id: Option<bool> - whether to convert numeric Ids to Node Id.
    ///
    pub fn set_numeric_edge_type_ids(
        mut self,
        numeric_edge_type_ids: Option<bool>,
    ) -> EdgeFileReader {
        if let Some(neti) = numeric_edge_type_ids {
            self.numeric_edge_type_ids = neti;
        }
        self
    }

    /// Set the numeric_id.
    ///
    /// # Arguments
    ///
    /// * numeric_id: Option<bool> - whether to convert numeric Ids to Node Id.
    ///
    pub fn set_numeric_node_ids(mut self, numeric_node_ids: Option<bool>) -> EdgeFileReader {
        if let Some(nni) = numeric_node_ids {
            self.numeric_node_ids = nni;
        }
        self
    }

    /// Set the ignore_duplicates.
    ///
    /// # Arguments
    ///
    /// * ignore_duplicates: Option<bool> - whether to ignore detected duplicates or raise exception.
    ///
    pub fn set_ignore_duplicates(mut self, ignore_duplicates: Option<bool>) -> EdgeFileReader {
        if let Some(v) = ignore_duplicates {
            self.reader.ignore_duplicates = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(
        mut self,
        separator: Option<S>,
    ) -> Result<EdgeFileReader, String> {
        if let Some(sep) = separator {
            let sep = sep.into();
            if sep.is_empty() {
                return Err("The separator cannot be empty.".to_owned());
            }
            self.reader.separator = sep;
        }
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> EdgeFileReader {
        if let Some(v) = header {
            self.reader.header = v;
        }
        self
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> EdgeFileReader {
        if let Some(v) = rows_to_skip {
            self.reader.rows_to_skip = v;
        }
        self
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    ///
    /// * max_rows_number: Option<u64> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(mut self, max_rows_number: Option<u64>) -> EdgeFileReader {
        self.reader.max_rows_number = max_rows_number;
        self
    }

    /// Return boolean representing if the edge types exist.
    pub fn has_edge_types(&self) -> bool {
        self.default_edge_type.is_some() || self.edge_types_column_number.is_some()
    }

    /// Return boolean representing if the weight types exist.
    pub fn has_weights(&self) -> bool {
        self.default_weight.is_some() || self.weights_column_number.is_some()
    }

    /// Parse a single line (vecotr of strings already splitted)
    /// # Arguments
    ///
    /// * vals: Vec<String> - Vector of the values of the line to be parsed
    fn parse_edge_line(&self, vals: Vec<Option<String>>) -> Result<StringQuadruple, String> {
        // extract the values
        let maybe_source_node_name = vals[self.sources_column_number].clone();
        let maybe_destination_node_name = vals[self.destinations_column_number].clone();
        if maybe_source_node_name.is_none() || maybe_destination_node_name.is_none() {
            return Err("Either the source or destination node ID are undefined.".to_string());
        }

        let source_node_name = maybe_source_node_name.unwrap();
        let destination_node_name = maybe_destination_node_name.unwrap();

        // Handle the extraction of the edge types.
        let maybe_edge_types_string = match self.edge_types_column_number {
            Some(column) => match vals[column].to_owned() {
                Some(edge_type) => Some(edge_type),
                None => self.default_edge_type.clone(),
            },
            None => self.default_edge_type.clone(),
        };

        // Handle the extraction of the weights.
        let maybe_weight_string = match self.weights_column_number {
            Some(column) => match vals[column].to_owned() {
                Some(w) => Some(parse_weight(w)?),
                None => self.default_weight,
            },
            None => self.default_weight,
        };

        Ok((
            source_node_name,
            destination_node_name,
            maybe_edge_types_string,
            maybe_weight_string,
        ))
    }

    /// Return iterator of rows of the edge file.
    pub fn read_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<StringQuadruple, String>> + '_, String> {
        if self.destinations_column_number == self.sources_column_number {
            return Err("The destinations column is the same as the sources one.".to_string());
        }
        if Some(self.destinations_column_number) == self.weights_column_number {
            return Err("The destinations column is the same as the weights one.".to_string());
        }
        if Some(self.sources_column_number) == self.weights_column_number {
            return Err("The sources column is the same as the weights one.".to_string());
        }
        if Some(self.sources_column_number) == self.edge_types_column_number {
            return Err("The sources column is the same as the edge types one.".to_string());
        }
        if Some(self.destinations_column_number) == self.edge_types_column_number {
            return Err("The destinations column is the same as the edge types one.".to_string());
        }
        if self.weights_column_number.is_some()
            && self.weights_column_number == self.edge_types_column_number
        {
            return Err("The weights column is the same as the edge types one.".to_string());
        }

        let expected_elements = self.reader.get_elements_per_line()?;
        if self.sources_column_number >= expected_elements {
            return Err(format!(
                concat!(
                    "The sources column number passed was {} but ",
                    "the first parsable line has {} values."
                ),
                self.sources_column_number, expected_elements
            ));
        }
        if self.destinations_column_number >= expected_elements {
            return Err(format!(
                concat!(
                    "The destinations column number passed was {} but ",
                    "the first parsable line has {} values."
                ),
                self.destinations_column_number, expected_elements
            ));
        }
        Ok(self
            .reader
            .read_lines()?
            .map(move |values| match values {
                Ok(vals) => self.parse_edge_line(vals),
                Err(e) => Err(e),
            })
            .filter_ok(move |(source_node_name, destination_node_name, _, _)| {
                !self.skip_self_loops || source_node_name != destination_node_name
            }))
    }
}

use super::*;

/// # Boolean Queries
/// The naming convection for unchecked methods follows:
/// - `is_X_by_Y`
/// - `has_X_by_Y`
impl Graph {
    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    ///
    /// `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_by_node_id(&self, node_id: NodeT) -> Result<bool, String> {
        Ok(self.has_singletons()
            && self.get_node_degree_by_node_id(node_id)? == 0
            && self
                .not_singleton_nodes
                .as_ref()
                .map_or(true, |nsns| !nsns[node_id as usize]))
    }

    /// Returns boolean representing if given node is a singleton with self-loops.
    ///
    /// # Arguments
    ///
    /// `node_id`: NodeT - The node to be checked for.
    pub fn is_singleton_with_self_loops_by_node_id(&self, node_id: NodeT) -> bool {
        self.singleton_nodes_with_self_loops
            .as_ref()
            .map_or(false, |snsls| snsls.contains(node_id))
    }

    /// Returns boolean representing if given node is a singleton.
    ///
    /// # Arguments
    /// `node_name`: &str - The node name to be checked for.
    pub fn is_singleton_by_node_name(&self, node_name: &str) -> Result<bool, String> {
        self.is_singleton_by_node_id(self.get_node_id_by_node_name(node_name)?)
    }

    /// Returns whether the graph has the given node name.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Examples
    /// To check if a node appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// let unexistent_node_name = "I_do_not_exist!";
    /// assert!(graph.has_node_by_node_name(node_name));
    /// assert!(!graph.has_node_by_node_name(unexistent_node_name));
    /// ```
    pub fn has_node_by_node_name(&self, node_name: &str) -> bool {
        self.get_node_id_by_node_name(node_name).is_ok()
    }

    /// Returns whether edge passing between given node ids exists.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Source node id.
    /// * `dst`: NodeT - Destination node id.
    ///
    /// # Examples
    /// To check if an edge appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph.has_edge_by_node_ids(0, 1));
    /// assert!(!graph.has_edge_by_node_ids(0, 4565));
    /// ```
    pub fn has_edge_by_node_ids(&self, src: NodeT, dst: NodeT) -> bool {
        self.get_edge_id_by_node_ids(src, dst).is_ok()
    }

    /// Returns whether edge with the given type passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - The source node of the edge.
    /// * dst: NodeT - The destination node of the edge.
    /// * edge_type: Option<EdgeTypeT> - The (optional) edge type.
    ///
    /// # Examples
    /// To check if an edge with given type appears in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph.has_edge_with_type_by_node_ids(0, 1, Some(0)));
    /// assert!(!graph.has_edge_with_type_by_node_ids(0, 1, Some(1)));
    /// ```
    pub fn has_edge_with_type_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> bool {
        self.get_edge_id_with_type_by_node_ids(src, dst, edge_type)
            .is_ok()
    }

    /// Returns boolean representing if given node is a trap.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Integer ID of the node, if this is bigger that the number of nodes it will panic.
    ///
    pub fn is_node_trap_by_node_id(&self, node_id: NodeT) -> Result<bool, String> {
        Ok(self.get_node_degree_by_node_id(node_id)? == 0
            && self
                .not_singleton_nodes
                .as_ref()
                .map_or(true, |nsns| nsns[node_id as usize]))
    }

    /// Returns whether the given node name and node type name exist in current graph.
    ///
    /// # Arguments
    ///
    /// * node_name: String - The node name.
    /// * node_type_name: String - The node type name.
    ///
    pub fn has_node_with_type_by_node_name(
        &self,
        node_name: &str,
        node_type_name: Option<Vec<String>>,
    ) -> bool {
        match self.get_node_id_by_node_name(node_name) {
            Err(_) => false,
            Ok(node_id) => {
                let our_node_types = self.get_node_type_name_by_node_id(node_id);
                match (our_node_types, node_type_name) {
                    (Err(_), None) => true,
                    (Ok(None), None) => true,
                    (Ok(Some(mut our_nts)), Some(mut other_nts)) => {
                        our_nts.sort();
                        other_nts.sort();
                        our_nts == other_nts
                    }
                    _ => false,
                }
            }
        }
    }

    /// Returns whether if edge passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: String - The source node name of the edge.
    /// * dst: String - The destination node name of the edge.
    ///
    /// # Examples
    /// To check if an edge in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// assert!(graph.has_edge_by_node_names("ENSP00000000233", "ENSP00000432568"));
    /// assert!(!graph.has_edge_by_node_names("ENSP00000000233", "NonExistent"));
    /// ```
    pub fn has_edge_by_node_names(&self, src_name: &str, dst_name: &str) -> bool {
        self.get_edge_id_by_node_names(src_name, dst_name).is_ok()
    }

    /// Returns whether if edge with type passing between given nodes exists.
    ///
    /// # Arguments
    ///
    /// * src: String - The source node name of the edge.
    /// * dst: String - The destination node name of the edge.
    /// * edge_type: Option<String> - The (optional) edge type name.
    ///
    /// # Examples
    /// To check if an edge with type in the graph you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let edge_type = "red".to_string();
    /// let unexistent_edge_type = "NonExistent".to_string();
    /// assert!(graph.has_edge_with_type_by_node_names("ENSP00000000233", "ENSP00000432568", Some(&edge_type)));
    /// assert!(!graph.has_edge_with_type_by_node_names("ENSP00000000233", "ENSP00000432568", Some(&unexistent_edge_type)));
    /// assert!(!graph.has_edge_with_type_by_node_names("ENSP00000000233", "NonExistent", Some(&edge_type)));
    /// assert!(!graph.has_edge_with_type_by_node_names("ENSP00000000233", "NonExistent", Some(&unexistent_edge_type)));
    /// ```
    pub fn has_edge_with_type_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> bool {
        self.get_edge_id_with_type_by_node_names(src_name, dst_name, edge_type_name)
            .is_ok()
    }
}

use super::*;
use roaring::RoaringBitmap;

/// # Drop.
impl Graph {
    /// Return a roaringbitmap with the node ids to keep.
    ///
    /// If both node\_names and node\_types are specified the result will be the
    /// union of both queries.
    ///
    /// # Arguments
    /// * `node_names` - The nodes to keep as strings
    /// * `node_types` - The nodes types to keep as strings
    ///
    pub(crate) fn get_filter_bitmap(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
    ) -> Result<Option<RoaringBitmap>, String> {
        let mut node_ids = RoaringBitmap::new();

        if let Some(ns) = node_names {
            node_ids.extend(
                ns.iter()
                    .map(|node_name| self.get_node_id_by_node_name(node_name))
                    .collect::<Result<Vec<NodeT>, String>>()?,
            );
        }

        if let Some(ndt) = node_types {
            let node_type_ids = self.get_node_type_ids_by_node_type_names(ndt)?;
            node_ids.extend(self.iter_node_ids().filter_map(|(node_id, nts)| {
                if nts.map_or_else(
                    //DEFAULT
                    || node_type_ids.contains(&None),
                    // If some
                    |ns| {
                        ns.into_iter()
                            .any(|node_type_name| node_type_ids.contains(&Some(node_type_name)))
                    },
                ) {
                    Some(node_id)
                } else {
                    None
                }
            }));
        }

        Ok(optionify!(node_ids))
    }
}

//! Test functions used both for testing and fuzzing.

use super::*;
use log::warn;
use rand::Rng;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const NONEXISTENT: &str = "Cthulhu is a fictional cosmic entity created by writer H. P. Lovecraft and first introduced in the short story The Call of Cthulhu,[2] published in the American pulp magazine Weird Tales in 1928. Considered a Great Old One within the pantheon of Lovecraftian cosmic entities, the creature has since been featured in numerous popular culture references. Lovecraft depicts it as a gigantic entity worshipped by cultists, in shape like an octopus, a dragon, and a caricature of human form. Its name was given to the Lovecraft-inspired universe where it and its fellow entities existed, the Cthulhu Mythos.";

// where to save the test files
#[cfg(target_os = "macos")]
static DEFAULT_PATH: &str = "/tmp/";
#[cfg(target_os = "linux")]
static DEFAULT_PATH: &str = "/tmp/";
#[cfg(target_os = "windows")]
static DEFAULT_PATH: &str = "";

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Computes a random string of the chosen length
pub fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Computes a random path.
pub fn random_path(path: Option<&str>) -> String {
    Path::new(path.unwrap_or(DEFAULT_PATH))
        .join(random_string(64))
        .to_str()
        .unwrap()
        .to_string()
}

#[allow(clippy::redundant_clone)]
/// Load the Strings Protein Protein Interaction graph with given parametrization.
/// This is our default graph we use on tests.
pub fn load_ppi(
    load_nodes: bool,
    load_edge_types: bool,
    load_weights: bool,
    directed: bool,
    verbose: bool,
    skip_self_loops: bool,
) -> Result<Graph, String> {
    let graph_name = "STRING PPI".to_owned();
    let nodes_reader = if load_nodes {
        Some(
            NodeFileReader::new("tests/data/ppi/nodes.tsv".to_string())?
                .set_verbose(Some(false))
                .set_node_types_column_number(Some(1))
                .set_nodes_column_number(Some(0))
                .set_node_types_column(Some("category".to_string()))?
                .set_default_node_type(Some("default".to_string()))
                .set_nodes_column(Some("id".to_string()))?
                .set_ignore_duplicates(Some(true))
                .set_separator(Some("\t"))
                .unwrap()
                .set_header(Some(true))
                .set_max_rows_number(Some(100000))
                .set_rows_to_skip(Some(0))
                .clone(),
        )
    } else {
        None
    };
    let edges_reader = EdgeFileReader::new("tests/data/ppi/edges.tsv".to_string())?
        .set_verbose(Some(verbose))
        .set_ignore_duplicates(Some(true))
        .set_separator(Some("\t"))
        .unwrap()
        .set_header(Some(true))
        .set_rows_to_skip(Some(0))
        .set_sources_column(Some("subject".to_string()))?
        .set_destinations_column(Some("object".to_string()))?
        .set_weights_column(if load_weights {
            Some("weight".to_string())
        } else {
            None
        })?
        .set_edge_types_column(if load_edge_types {
            Some("edge_label".to_string())
        } else {
            None
        })?
        .set_default_edge_type(if load_edge_types {
            Some("Kebab".to_string())
        } else {
            None
        })
        .set_max_rows_number(Some(100000))
        .set_default_weight(if load_weights { Some(5.0) } else { None })
        .set_skip_self_loops(Some(skip_self_loops))
        .clone();

    let ppi = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        directed,
        false,
        graph_name.clone(),
    );
    assert!(
        ppi.is_ok(),
        "We expect to be able to load the String PPI graph."
    );
    let ppi = ppi?;
    assert_eq!(ppi.has_node_types(), load_nodes);
    assert_eq!(ppi.has_edge_types(), load_edge_types,);
    assert_eq!(ppi.has_weights(), load_weights);
    assert_eq!(
        ppi.has_selfloops(),
        !skip_self_loops,
        concat!(
            "I was expecting the graph self-loops status to be {} ",
            "since we have given parameter skip_self_loops equal to {}, ",
            "but actually is {}.\n",
            "The graph report is: \n {:?}"
        ),
        !skip_self_loops,
        skip_self_loops,
        ppi.has_selfloops(),
        ppi.textual_report(false)
    );
    Ok(ppi)
}

#[allow(clippy::redundant_clone)]
/// This is our default graph we use on tests with node types.
pub fn load_cora() -> Result<Graph, String> {
    let graph_name = "Cora".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/cora/edges.tsv")?
        .set_separator(Some("\t"))?
        .set_verbose(Some(false))
        .set_sources_column(Some("subject"))?
        .set_destinations_column(Some("object"))?
        .set_edge_types_column(Some("edge_type"))?;
    let nodes_reader = Some(
        NodeFileReader::new("tests/data/cora/nodes.tsv")?
            .set_separator(Some("\t"))?
            .set_nodes_column(Some("id"))?
            .set_verbose(Some(false))
            .set_node_types_column(Some("node_type"))?,
    );
    let cora =
        Graph::from_unsorted_csv(edges_reader, nodes_reader, false, false, graph_name.clone())?;
    Ok(cora)
}

/// Return WalksParameters to execute a first order walk.
pub fn first_order_walker(graph: &Graph) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(8)?
        .set_iterations(Some(1))?
        .set_random_state(Some(43))
        .set_dense_node_mapping(Some(graph.get_dense_node_mapping())))
}

/// Return WalksParameters to execute a second order walk.
pub fn second_order_walker(
    graph: &Graph,
    return_weight: WeightT,
    explore_weight: WeightT,
) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(8)?
        .set_iterations(Some(1))?
        .set_return_weight(Some(return_weight))?
        .set_explore_weight(Some(explore_weight))?
        .set_max_neighbours(Some(3))?
        .set_change_edge_type_weight(Some(2.0))?
        .set_change_node_type_weight(Some(2.0))?
        .set_dense_node_mapping(Some(graph.get_dense_node_mapping()))
        .set_random_state(Some(43)))
}

fn validate_vocabularies(graph: &Graph) {
    if let Some(ets) = &graph.edge_types {
        assert_eq!(!ets.ids.is_empty(), graph.has_edge_types());
    }

    if let Some(nts) = &graph.node_types {
        assert_eq!(!nts.ids.is_empty(), graph.has_node_types());
    }

    if let Some(ws) = &graph.weights {
        assert_eq!(
            !ws.is_empty(), graph.has_weights(),
            concat!(
                "We expect the edge weights vector to NOT be empty if the graph says it has weights.\n",
                "The graph report is:\n{:?}"
            ),
            graph.textual_report(false)
        );
    }
}

/// Executes the default test suite for holdouts.
pub fn default_holdout_test_suite(
    graph: &Graph,
    train: &Graph,
    test: &Graph,
) -> Result<(), String> {
    for g in &[graph, train, test] {
        validate_vocabularies(g);
    }
    assert!(
        !train.overlaps(&test)?,
        "Training graph overlaps with test graph!"
    );
    assert!(
        !test.overlaps(&train)?,
        "Test graph overlaps with training graph!"
    );
    assert!(graph.contains(&train)?, "Graph does not training graph.");
    assert!(graph.contains(&test)?, "Graph does not contain test graph.");
    let summed = (train | test)?;
    validate_vocabularies(&summed);
    assert!(
        summed.contains(&graph)?,
        "Composed train and test graph do not contained original graph."
    );
    let subtracted = (graph - test)?;
    validate_vocabularies(&subtracted);
    assert!(
        subtracted.contains(&train)?,
        "Main graph subtracted test does not contain training graph."
    );
    assert!(
        !subtracted.overlaps(&test)?,
        "Main graph subtracted train does not contain test graph."
    );
    let xorred = (graph ^ test)?;
    validate_vocabularies(&xorred);
    assert!(
        xorred.contains(&train)?,
        "Main graph xorred test does not contain training graph."
    );
    assert!(
        !xorred.overlaps(&test)?,
        "Main graph xorred train does not contain testing graph."
    );
    let anded = (graph & test)?;
    validate_vocabularies(&anded);
    assert!(
        anded.contains(&test)?,
        "Main graph anded test does not contain training graph."
    );
    Ok(())
}

/// Test that the spanning arborescence algorithm from bader is working correctly.
pub fn test_spanning_arborescence_bader(graph: &Graph, verbose: bool) {
    let kruskal_tree = graph.spanning_arborescence_kruskal(verbose).0;
    let random_kruskal_tree = graph
        .random_spanning_arborescence_kruskal(42, &None, verbose)
        .0;
    if !graph.directed {
        let spanning_arborescence_bader: Vec<(NodeT, NodeT)> =
            graph.spanning_arborescence(verbose).unwrap().1.collect();
        assert_eq!(
            spanning_arborescence_bader.len(), kruskal_tree.len(),
            "The number of extracted edges forming the spanning arborescence computed by the bader's algorithm does not match the one computed by kruskal. The graph report is:\n{:?}\nThe bader's tree is:\n{:?}\nThe kruskal's tree is:\n{:?}",
            graph.textual_report(false), spanning_arborescence_bader, kruskal_tree,
        );
    } else {
        assert!(graph.spanning_arborescence(verbose).is_err());
    }
    assert_eq!(random_kruskal_tree.len() as usize, kruskal_tree.len());
}

pub fn test_graph_properties(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    // Testing that vocabularies are properly loaded
    validate_vocabularies(graph);

    // Test get_edge_id_with_type_by_node_names()
    assert!(
        graph
            .get_edge_id_with_type_by_node_names(NONEXISTENT, NONEXISTENT, None)
            .is_err(),
        "Graph contains non-existing edge."
    );

    // Test has_node_by_name
    assert!(
        !(graph.has_node_with_type_by_node_name(NONEXISTENT, None)),
        "The graph seems to have a non-existing node."
    );
    assert!(
        !(graph.has_node_by_node_name(NONEXISTENT)),
        "The graph seems to have a non-existing node."
    );

    // Test translate_edge|node_types()
    assert!(
        graph
            .get_edge_type_ids_by_edge_type_names(vec![Some(NONEXISTENT.to_string())])
            .is_err(),
        "The graph seems to have a non-existing edge type."
    );

    assert!(
        graph
            .get_node_type_ids_by_node_type_names(vec![Some(NONEXISTENT.to_string())])
            .is_err(),
        "The graph seems to have a non-existing node type."
    );

    assert_eq!(
        graph.get_not_singleton_nodes_number() + graph.get_singleton_nodes_number(),
        graph.get_nodes_number(),
        "Sum of singleton and non singleton nodes number does not match."
    );

    warn!("Running connected components tests.");
    let (_components_number, smallest, biggest) = graph.connected_components_number(false);
    assert!(
        biggest >= smallest,
        "smallest: {} biggest: {}",
        smallest,
        biggest
    );

    if smallest == 1 {
        assert!(
            graph.has_singletons() || graph.has_singleton_nodes_with_self_loops(),
            "When the smallest component is one the graph must have singletons! Graph report: \n{:?}",
            graph.textual_report(false)
        );
    }

    if smallest == 0 {
        assert!(
            !graph.has_nodes(),
            "When the smallest component is zero the graph must be empty! Graph report: \n{:?}",
            graph.textual_report(false)
        );
    }
    // Get one edge from the graph if there are any presents
    if let Some(edge) = graph.iter_unique_edges(true).next() {
        let src_string = graph.get_node_name_by_node_id(edge.0).unwrap();
        let dst_string = graph.get_node_name_by_node_id(edge.1).unwrap();
        let edge_id = graph.get_edge_id_by_node_names(&src_string, &dst_string)?;
        if graph.has_edge_types() {
            let edge_type = graph.get_edge_type_name_by_edge_id(edge_id)?;
            assert!(
                graph.has_edge_with_type_by_node_names(&src_string, &dst_string, edge_type.as_ref()),
                "I was expecting for the edge ({}, {}, {:?}) to exist, but it seems to not exist in graph {:?}",
                src_string,
                dst_string,
                edge_type,
                graph.textual_report(false)
            );
        } else {
            assert!(
                graph.has_edge_by_node_names(&src_string, &dst_string),
                "I was expecting for the edge ({}, {}) without type to exist, but it seems to not exist in graph {:?}",
                src_string,
                dst_string,
                graph.textual_report(false)
            );
        }
        assert!(
            graph.has_node_by_node_name(&src_string) && graph.has_node_by_node_name(&dst_string)
        );
        if graph.has_node_types() {
            assert!(
                graph.has_node_with_type_by_node_name(
                    &src_string,
                    graph.get_node_type_name_by_node_name(&src_string)?
                ) && graph.has_node_with_type_by_node_name(
                    &dst_string,
                    graph.get_node_type_name_by_node_name(&dst_string)?
                ),
                concat!(
                    "The nodes {:?} and {:?} with node types are not present in the graph.\n",
                    "The node types are {:?} and {:?}.\n",
                    "The first node existance is {}\n",
                    "The second node existance is {}\n",
                    "The graph report is {:?}"
                ),
                src_string,
                dst_string,
                graph.get_node_type_name_by_node_name(&src_string),
                graph.get_node_type_name_by_node_name(&dst_string),
                graph.has_node_with_type_by_node_name(
                    &src_string,
                    graph.get_node_type_name_by_node_name(&src_string)?
                ),
                graph.has_node_with_type_by_node_name(
                    &dst_string,
                    graph.get_node_type_name_by_node_name(&dst_string)?
                ),
                graph.textual_report(false)
            );
        }
        assert_eq!(
            graph.get_edge_id_by_node_names(&src_string, &dst_string)?,
            graph.get_edge_id_by_node_ids(edge.0, edge.1).unwrap(),
            "Check of given edge ID does not match."
        );
    }

    // Test the generation of the textual report, this includes the connected components algorithm.
    graph.report();
    graph.textual_report(verbose)?;

    // Compute degrees metrics
    for src in 0..5 {
        for dst in 0..5 {
            let _ = graph.degrees_product(src, dst);
            let _ = graph.jaccard_index(src, dst);
            let _ = graph.adamic_adar_index(src, dst);
            let _ = graph.resource_allocation_index(src, dst);
        }
    }

    assert_eq!(
        graph.has_node_types(),
        graph.get_node_type_id_by_node_id(0).is_ok()
    );

    assert!(
        graph.get_node_type_id_by_node_id(graph.get_nodes_number() + 1).is_err(),
        "Given graph does not raise an exception when a node's node type greater than the number of available nodes is requested."
    );

    assert_eq!(
        graph.has_edge_types(),
        graph.get_edge_type_id_by_edge_id(0).is_ok()
    );

    assert!(
        graph.get_edge_type_id_by_edge_id(graph.get_directed_edges_number() + 1).is_err(),
        "Given graph does not raise an exception when a edge's edge type greater than the number of available edges is requested."
    );

    // Evaluate get_node_type
    assert_eq!(
        graph.get_node_type_id_by_node_id(0).is_ok(),
        graph.has_node_types()
    );

    // Evaluate get_edge_type
    assert_eq!(
        graph.get_edge_type_id_by_edge_id(0).is_ok(),
        graph.has_edge_types()
    );

    // Evaluate get_node_type_counts
    assert_eq!(graph.get_node_type_counts().is_ok(), graph.has_node_types());

    // Evaluate get_edge_type_counts
    assert_eq!(graph.get_edge_type_counts().is_ok(), graph.has_edge_types());

    // Evaluate get_edge_type_counts_hashmap
    assert_eq!(
        graph.get_edge_type_counts_hashmap().is_ok(),
        graph.has_edge_types()
    );

    graph.set_name(graph.get_name());
    graph.strongly_connected_components();

    // Checking that the connected components are a dense range.
    let (_, connected_components, total_connected_components, _, _) =
        graph.random_spanning_arborescence_kruskal(42, &None, verbose);
    let max_component_id = connected_components.iter().max();
    if let Some(mci) = max_component_id {
        assert_eq!(
            *mci as usize,
            total_connected_components as usize - 1,
            "We expected the connected components to be a dense set.\n The obtained components are: \n{:?}\n The graph report is:\n{:?}",
            connected_components,
            graph.textual_report(true)
        );
    }

    Ok(())
}

pub fn test_random_walks(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    // Testing principal random walk algorithms
    let walker = first_order_walker(&graph)?;
    assert_eq!(walker.clone(), walker);
    let walker2 = second_order_walker(&graph, 2.0, 2.0)?;
    assert_eq!(walker2.clone(), walker2);

    if !graph.directed {
        warn!("Executing random walks tests.");
        for mode in 0..3 {
            if mode == 1 {
                graph.enable(false, true, true, None)?;
                if let Some(outbounds) = &graph.outbounds {
                    assert_eq!(
                        outbounds.len(),
                        graph.get_nodes_number() as usize,
                        "Length of outbounds does not match number of nodes in the graph."
                    );
                }
                if let Some(destinations) = &graph.destinations {
                    assert_eq!(
                        destinations.len(),
                        graph.get_directed_edges_number() as usize,
                        "Length of destinations does not match number of edges in the graph."
                    );
                }
            }
            if mode == 2 {
                graph.enable(false, false, false, Some(0.05))?;
                assert!(
                    graph.cached_destinations.is_some(),
                    "Cached destinations are not None when cache is enabled."
                );
            }
            assert_eq!(
                graph
                    .random_walks_iter(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .random_walks_iter(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of first order are not reproducible!"
            );

            assert_eq!(
                graph
                    .random_walks_iter(1, &second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .random_walks_iter(1, &second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of second order are not reproducible!"
            );

            assert_eq!(
                graph
                    .complete_walks_iter(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .complete_walks_iter(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete first order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .complete_walks_iter(&second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .complete_walks_iter(&second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .complete_walks_iter(&second_order_walker(&graph, 2.0, 1.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .complete_walks_iter(&second_order_walker(&graph, 2.0, 1.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .complete_walks_iter(&second_order_walker(&graph, 1.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .complete_walks_iter(&second_order_walker(&graph, 1.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );
        }
    } else {
        assert!(graph.complete_walks_iter(&walker).is_err());
    }
    Ok(())
}

pub fn test_edge_holdouts(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    for include_all_edge_types in &[false, true] {
        let (train, test) =
            graph.random_holdout(4, 0.6, *include_all_edge_types, None, None, verbose)?;
        default_holdout_test_suite(graph, &train, &test)?;
        let (train, test) =
            graph.connected_holdout(4, 0.8, None, *include_all_edge_types, verbose)?;
        let (total, min_comp, max_comp) = graph.connected_components_number(verbose);
        assert_eq!(
            graph.connected_components_number(verbose),
            train.connected_components_number(verbose),
            "The number of components of the original graph and the connected training set does not match. Particularly, the number of nodes in the graph is {nodes_number}.",
            nodes_number=graph.get_nodes_number().to_string()
        );
        if total == 1 {
            assert_eq!(
                min_comp,
                graph.get_nodes_number(),
                concat!(
                    "We expect for the minimum size of connected components ",
                    "in a graph with a single connected component to ",
                    "match the number of nodes of the graph, but we got ",
                    "the minimum component with size {} and the number ",
                    "of nodes in the graph equal to {}.\n",
                    "The graph report is: \n {:?}",
                ),
                min_comp,
                graph.get_nodes_number(),
                graph.textual_report(false)
            );
            assert_eq!(max_comp, graph.get_nodes_number());
            assert_eq!(min_comp, test.get_nodes_number());
            assert_eq!(max_comp, test.get_nodes_number());
        }
        if total == 2 {
            assert_eq!(
                max_comp + min_comp, graph.get_nodes_number(),
                "We expected that the number of the minimum component ({}) plus the maximum component ({}), when the components are two, made up the graph nodes ({}).\nThe graph report is:\n {:?}",
                min_comp, max_comp, graph.get_nodes_number(),
                graph.textual_report(false)
            );
            assert_eq!(max_comp + min_comp, test.get_nodes_number());
        }
        default_holdout_test_suite(graph, &train, &test)?;
    }
    Ok(())
}

pub fn test_remove_components(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    if graph.connected_components_number(verbose).0 > 1 {
        let without_selfloops = graph.remove(
            None, None, None, None, None, None, None, None, false, false, false, false, true,
            verbose,
        )?;

        assert_eq!(
            graph.connected_components_number(verbose),
            without_selfloops.connected_components_number(verbose),
            concat!(
                "We expected the graph to have the same components once we remove the selfloops.\n",
                "The report of the original graph is {:?}\n",
                "The report of the filtered graph is {:?}\n",
            ),
            graph.textual_report(false),
            without_selfloops.textual_report(false),
        );

        let single_component = graph.remove_components(None, None, None, None, Some(1), verbose);
        assert!(
            single_component.is_ok(),
            concat!(
                "Removing all the components except the first one returned an error.\n",
                "The error is:\n{:?}\nand the graph report is:\n{:?}"
            ),
            single_component,
            graph.textual_report(false)
        );
        let single_component_number = single_component
            .unwrap()
            .connected_components_number(verbose)
            .0;
        assert_eq!(
            single_component_number,
            1,
            concat!(
                "Removing all the components except the first one returned a graph ",
                "with {} components, which is not one.\nThe report of the graph is:{:?}\n"
            ),
            single_component_number,
            graph.textual_report(false)
        );

        let test = graph.remove_components(
            Some(vec![graph.nodes.unchecked_translate(0)]),
            None,
            None,
            None,
            None,
            verbose,
        )?;
        let no_selfloops = test.remove(
            None, None, None, None, None, None, None, None, false, false, false, false, true,
            verbose,
        )?;
        assert_eq!(
            no_selfloops.connected_components_number(verbose).0,
            1,
            concat!(
                "Expected number of components (1) is not matched!\n",
                "The report of the original graph is {:?}\n",
                "The report of the graph with only one component is {:?}\n",
                "The report of the graph without selfloops is {:?}\n",
            ),
            graph.textual_report(false),
            test.textual_report(false),
            no_selfloops.textual_report(false)
        );
        if let Ok(node_type_name) = graph.get_node_type_name_by_node_type_id(0) {
            assert!(graph
                .remove_components(
                    None,
                    Some(vec![Some(node_type_name)]),
                    None,
                    None,
                    None,
                    verbose
                )
                .is_ok());
        }
        if graph.has_unknown_node_types() {
            let without_unknowns =
                graph.remove_components(None, Some(vec![None]), None, None, None, verbose);
            assert!(
                without_unknowns.is_ok(),
                "Could not remove components without node type None.\nThe error is {:?}\nThe graph report is {:?}",
                without_unknowns, graph.textual_report(false)
            );
        }
        if let Ok(edge_type_name) = graph.get_edge_type_name_by_edge_type_id(0) {
            assert!(graph
                .remove_components(
                    None,
                    None,
                    Some(vec![Some(edge_type_name)]),
                    None,
                    None,
                    verbose
                )
                .is_ok());
        }
        if graph.has_unknown_edge_types() {
            assert!(graph
                .remove_components(None, None, Some(vec![None]), None, None, verbose)
                .is_ok());
        }
    } else {
        assert!(
            graph
                .remove_components(None, None, None, None, None, verbose)
                .is_ok(),
            "We expect it to be possible, now, to create empty graphs."
        );
    }

    Ok(())
}

pub fn test_kfold(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    let k = 3;
    for i in 0..k {
        let (train, test) = graph.kfold(k, i, None, 42, false)?;
        assert!(
            test.get_edges_number() <= (graph.get_edges_number() / k) + 1,
            concat!(
                "Check that test kfolds respect size bound has failed!\n",
                "The value of k is {}.\n",
                "The report of the original graph is:\n{:?}\n",
                "The report of the train graph is:\n{:?}\n",
                "The report of the test graph is:\n{:?}\n",
                "We expect that the test graph has at most {} edges but it has {}.\n",
                "The holdout index is {}.\n",
            ),
            k,
            graph.textual_report(false),
            train.textual_report(false),
            test.textual_report(false),
            (graph.get_edges_number() / k) + 1,
            test.get_edges_number(),
            i
        );
        default_holdout_test_suite(graph, &train, &test)?;
    }

    if let Ok(edge_t) = graph.get_edge_type_name_by_edge_type_id(0) {
        for i in 0..k {
            let (train, test) = graph.kfold(k, i, Some(vec![Some(edge_t.clone())]), 1337, false)?;
            default_holdout_test_suite(graph, &train, &test)?;
        }
    }

    Ok(())
}

pub fn test_negative_edges_generation(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    for only_from_same_component in &[true, false] {
        let negatives = graph.sample_negatives(
            4,
            graph.get_edges_number(),
            None,
            *only_from_same_component,
            true,
        )?;
        assert_eq!(
            graph.get_edges_number(),
            negatives.get_edges_number(),
            "We expect the graph and its negative graph to have the same number of edges but we got {} and {}.",
            graph.get_edges_number(),
            negatives.get_edges_number()
        );
        validate_vocabularies(&negatives);
        if !graph.has_edge_types() {
            assert!(!graph.overlaps(&negatives)?);
            assert!(!negatives.overlaps(&graph)?);
        }
        // Testing holdouts executed on negative edges.
        let (neg_train, neg_test) =
            negatives.random_holdout(32, 0.8, false, None, None, verbose)?;

        neg_test.get_trap_nodes_number();

        default_holdout_test_suite(&negatives, &neg_train, &neg_test)?;
    }

    Ok(())
}

pub fn test_subgraph_generation(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    let expected_nodes = graph.get_not_singleton_nodes_number() / 10;
    let subgraph = graph.random_subgraph(6, expected_nodes, verbose)?;
    assert!(subgraph.overlaps(&graph)?);
    assert!(subgraph.get_not_singleton_nodes_number() <= expected_nodes + 1);
    Ok(())
}

pub fn test_dump_graph(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    let node_file = random_path(None);
    let nodes_writer = NodeFileWriter::new(node_file.clone())
        .set_verbose(Some(verbose))
        .set_separator(Some("\t"))
        .set_header(Some(true))
        .set_node_types_column_number(Some(4))
        .set_nodes_column_number(Some(6))
        .set_node_types_column(Some("node_types"))
        .set_nodes_column(Some("node_column".to_string()));
    nodes_writer.dump(&graph)?;
    fs::remove_file(node_file).unwrap();

    let edges_file = random_path(None);
    let edges_writer = EdgeFileWriter::new(edges_file.clone())
        .set_verbose(Some(verbose))
        .set_separator(Some("\t"))
        .set_header(Some(true))
        .set_edge_types_column(Some("edge_types"))
        .set_destinations_column_number(Some(3))
        .set_weights_column(Some("weight".to_string()))
        .set_weights_column_number(Some(2))
        .set_sources_column(Some("The land of sushi".to_string()))
        .set_sources_column_number(Some(0))
        .set_destinations_column(Some("The land of pizza".to_string()))
        .set_destinations_column_number(Some(1));

    edges_writer.dump(&graph)?;
    fs::remove_file(edges_file).unwrap();

    Ok(())
}

pub fn test_embiggen_preprocessing(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    let walker = first_order_walker(&graph)?;
    if !graph.directed {
        let (terms_number, iterator) = graph.cooccurence_matrix(&walker, 3, verbose)?;
        assert_eq!(terms_number, iterator.count());

        let window_size = 3;
        let batch_size = 256;
        let data = graph
            .node2vec(&walker, batch_size, window_size)?
            .collect::<Vec<_>>();
        assert_eq!(
            data.len(),
            batch_size as usize
                * walker.iterations as usize
                * (walker.single_walk_parameters.walk_length as usize - window_size * 2)
        );
        for (context, _) in data.iter() {
            assert_eq!(context.len(), window_size * 2);
        }
    }
    if graph.has_edges() {
        graph
            .link_prediction_degrees(0, 256, true, 10.0, false, 10, &None)
            .unwrap()
            .collect::<Vec<_>>();
        graph
            .link_prediction_ids(0, 256, 10.0, false, 10, &None)
            .unwrap()
            .collect::<Vec<_>>();
    }

    Ok(())
}

pub fn test_graph_filter(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    assert!(graph
        .filter(
            Some(graph.get_node_names()),
            graph
                .get_node_type_names()
                .map(|ntn| ntn.into_iter().map(Option::Some).collect()),
            graph
                .get_edge_type_names()
                .map(|etn| etn.into_iter().map(Option::Some).collect()),
            Some(1000.0),
            Some(10.0),
            verbose,
        )
        .is_err());
    let _ = graph.filter(
        Some(graph.get_node_names()),
        graph
            .get_node_type_names()
            .map(|ntn| ntn.into_iter().map(Option::Some).collect()),
        graph
            .get_edge_type_names()
            .map(|etn| etn.into_iter().map(Option::Some).collect()),
        graph.get_min_weight().ok(),
        graph.get_max_weight().ok(),
        verbose,
    );
    Ok(())
}

pub fn test_edgelist_generation(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    let _clique = graph.get_clique_edge_names(
        None,
        None,
        Some(false),
        None,
        // limit to compute the clique for at most the first 3 nodes
        // because it's really expensive computationally.
        Some(
            graph
                .get_node_names()
                .iter()
                .take(3)
                .cloned()
                .collect::<HashSet<String>>(),
        ),
    );
    warn!("Running edge lists generator tests.");
    if graph.get_nodes_number() > 1 {
        let _bipartite = graph.get_bipartite_edge_names(
            None,
            Some(
                [graph.get_node_name_by_node_id(0).unwrap()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            Some(
                [graph.get_node_name_by_node_id(1).unwrap()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
            None,
        )?;
        let _star = graph.get_star_edges(
            graph.get_node_name_by_node_id(0).unwrap(),
            Some(false),
            Some(
                [graph.get_node_name_by_node_id(1).unwrap()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
        let _star = graph.get_star_edge_names(
            graph.get_node_name_by_node_id(0).unwrap(),
            Some(false),
            Some(
                [graph.get_node_name_by_node_id(1).unwrap()]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
    }
    Ok(())
}

pub fn test_nodelabel_holdouts(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    for use_stratification in [true, false].iter() {
        if *use_stratification
            && (graph.has_multilabel_node_types() || graph.get_minimum_node_types_number() < 2)
            || graph.get_nodes_number() - graph.get_unknown_node_types_number() < 2
            || !graph.has_node_types()
        {
            assert!(graph
                .node_label_holdout(0.8, *use_stratification, 42)
                .is_err());
            continue;
        }

        let (train, test) = graph.node_label_holdout(0.8, *use_stratification, 42)?;
        let remerged = &mut (&train | &test)?;
        assert_eq!(remerged.node_types, graph.node_types);
        assert!(
            remerged.contains(graph)?,
            "The re-merged holdouts does not contain the original graph."
        );
        assert!(
            graph.contains(remerged)?,
            "The re-merged holdouts does not contain the original graph."
        );
        assert!(
            train.node_types.as_ref().map_or(false, |train_nts| {
                test.node_types.as_ref().map_or(false, |test_nts| {
                    train_nts.ids.iter().zip(test_nts.ids.iter()).all(
                        |(train_node_type, test_node_type)| {
                            !(train_node_type.is_some() && test_node_type.is_some())
                        },
                    )
                })
            }),
            "The train and test node-label graphs are overlapping!"
        );
    }
    Ok(())
}

pub fn test_edgelabel_holdouts(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    for use_stratification in [true, false].iter() {
        if *use_stratification && graph.get_minimum_edge_types_number() < 2
            || graph.get_directed_edges_number() - graph.get_unknown_edge_types_number() < 2
            || !graph.has_edge_types()
        {
            assert!(graph
                .edge_label_holdout(0.8, *use_stratification, 42)
                .is_err());
            continue;
        }
        let (train, test) = graph.edge_label_holdout(0.8, *use_stratification, 42)?;
        assert!(
            train.edge_types.as_ref().map_or(false, |train_nts| {
                test.edge_types.as_ref().map_or(false, |test_nts| {
                    train_nts.ids.iter().zip(test_nts.ids.iter()).all(
                        |(train_edge_type, test_edge_type)| {
                            !(train_edge_type.is_some() && test_edge_type.is_some())
                        },
                    )
                })
            }),
            "The train and test edge-label graphs are overlapping!"
        );
    }
    Ok(())
}

pub fn test_graph_removes(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    {
        let without_edge_types = graph.remove(
            None, None, None, None, None, None, None, None, false, false, true, false, false,
            verbose,
        );
        if let Some(we) = &without_edge_types.ok() {
            validate_vocabularies(we);
            assert_eq!(we.has_edge_types(), false);
            assert_eq!(we.has_weights(), graph.has_weights());
            assert_eq!(we.node_types, graph.node_types);
            assert_eq!(
                we.get_unique_edges_number(),
                graph.get_unique_edges_number(),
                concat!(
                    "Number of unique edges does not match in graph without edge types.\n",
                    "The report of the original graph is \n{:?}\n",
                    "The report of the graph without edge types is \n{:?}",
                ),
                graph.textual_report(false),
                we.textual_report(false),
            );
            assert_eq!(
                we.get_unique_self_loop_number(),
                graph.get_unique_self_loop_number(),
                "Number of unique self loops does not match in graph without edge types."
            );
            assert_eq!(we.nodes, graph.nodes);
        }
    }
    {
        let without_node_types = graph.remove(
            None, None, None, None, None, None, None, None, false, true, false, false, false,
            verbose,
        );
        if let Some(wn) = &without_node_types.ok() {
            validate_vocabularies(wn);
            assert_eq!(wn.has_node_types(), false);
            assert_eq!(
                wn.weights,
                graph.weights,
                concat!(
                    "We expected the weights not to change when removig node types.",
                    "\nThe report of the original graph is {:?}.",
                    "\nThe report of the filtered graph is {:?}."
                ),
                graph.textual_report(false),
                wn.textual_report(false)
            );
            assert_eq!(wn.has_selfloops(), graph.has_selfloops());
            assert_eq!(wn.nodes, graph.nodes);
        }
    }
    {
        let without_weights = graph.remove(
            None, None, None, None, None, None, None, None, true, false, false, false, false,
            verbose,
        );
        if let Some(ww) = &without_weights.ok() {
            validate_vocabularies(ww);
            assert_eq!(ww.has_weights(), false);
            assert_eq!(ww.node_types, graph.node_types);
            assert_eq!(ww.has_selfloops(), graph.has_selfloops());
            assert_eq!(ww.nodes, graph.nodes);
        }
    }

    Ok(())
}

pub fn test_clone_and_setters(graph: &mut Graph, _verbose: bool) -> Result<(), String> {
    let mut clone = graph.clone();
    clone = clone.set_all_edge_types("TEST_SET_ALL_EDGE_TYPES");
    clone = clone.set_all_node_types("TEST_SET_ALL_NODE_TYPES");

    assert_eq!(
        clone.get_edge_types_number(),
        1,
        "Number of edge types of the graph is not 1."
    );
    assert_eq!(
        clone.get_unchecked_edge_count_by_edge_type_id(Some(0)),
        graph.get_directed_edges_number(),
        "Number of edges with the unique edge type does not match number of edges in the graph."
    );

    assert_eq!(
        clone.get_node_types_number(),
        1,
        "Number of node types of the graph is not 1."
    );
    assert_eq!(
        clone.get_unchecked_node_count_by_node_type_id(Some(0)),
        graph.get_nodes_number(),
        "Number of nodes with the unique node type does not match number of nodes in the graph."
    );

    Ok(())
}

pub fn test_graph_remapping(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    assert!(
        graph.are_nodes_remappable(&graph),
        "Graph always should be remappable to itself."
    );
    assert!(
        graph.remap(&graph, verbose).is_ok(),
        "Graph always should be remappable to itself."
    );
    Ok(())
}

/// Executes near-complete test of all functions for the given graph.
fn _default_test_suite(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    warn!("Starting default test suite.");
    let _ = test_graph_properties(graph, verbose);

    warn!("Testing SkipGram / CBOW / GloVe preprocessing.");
    let _ = test_embiggen_preprocessing(graph, verbose);

    warn!("Testing subgraph generation.");
    let _ = test_subgraph_generation(graph, verbose);

    warn!("Testing clone and setters.");
    let _ = test_clone_and_setters(graph, verbose);

    warn!("Testing edge-label holdouts tests.");
    let _ = test_edgelabel_holdouts(graph, verbose);

    warn!("Testing writing out graph to file.");
    let _ = test_dump_graph(graph, verbose);

    warn!("Testing generic filtering mechanism.");
    let _ = test_graph_filter(graph, verbose);

    warn!("Testing the spanning arborescences.");
    let _ = test_spanning_arborescence_bader(graph, verbose);

    warn!("Running node-label holdouts tests.");
    let _ = test_nodelabel_holdouts(graph, verbose);

    warn!("Running remove components tests.");
    let _ = test_remove_components(graph, verbose);

    warn!("Testing removes.");
    let _ = test_graph_removes(graph, verbose);

    warn!("Testing negative edges generation.");
    let _ = test_negative_edges_generation(graph, verbose);

    warn!("Executing edge holdouts tests.");
    let _ = test_edge_holdouts(graph, verbose);

    warn!("Testing k-fold holdouts.");
    let _ = test_kfold(graph, verbose);

    warn!("Testing edge lists generation.");
    let _ = test_edgelist_generation(graph, verbose);

    warn!("Testing graph remapping.");
    let _ = test_graph_remapping(graph, verbose);

    warn!("Testing random walks.");
    let _ = test_random_walks(graph, verbose);

    Ok(())
}

/// Executes near-complete test of all functions for the given graph.
pub fn default_test_suite(graph: &mut Graph, verbose: bool) -> Result<(), String> {
    warn!("Starting default test suite.");
    let _ = _default_test_suite(graph, verbose);
    warn!("Starting default test suite with speedups enabled.");
    graph.enable(true, true, true, None)?;
    let _ = _default_test_suite(graph, verbose);
    Ok(())
}

use super::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;

impl Graph {
    /// Enable extra perks that buys you time as you accept to spend more memory.
    ///
    /// # Arguments
    /// * `vector_sources`: bool, whether to cache sources into a vector for faster walks.
    /// * `vector_destinations`: bool, whether to cache destinations into a vector for faster walks.
    /// * `vector_outbounds`: bool, whether to cache outbounds into a vector for faster walks.
    /// * `cache_size`: Option<f64>, percentage of nodes destinations to cache. This cannot be used with the vector destinations.
    pub fn enable(
        &mut self,
        vector_sources: bool,
        vector_destinations: bool,
        vector_outbounds: bool,
        cache_size: Option<f64>,
    ) -> Result<(), String> {
        if vector_destinations {
            if self.destinations.is_none() {
                self.destinations = Some(self.get_destinations(true));
            }
        } else {
            self.destinations = None;
        }
        if vector_sources {
            if self.sources.is_none() {
                self.sources = Some(self.get_sources(true));
            }
        } else {
            self.sources = None;
        }
        if vector_outbounds {
            if self.outbounds.is_none() {
                self.outbounds = Some(self.get_outbounds());
            }
        } else {
            self.outbounds = None;
        }
        if let Some(cs) = cache_size {
            if vector_destinations {
                return Err("You cannot use cache if you enable the destinations vector".to_owned());
            }
            if cs <= 0.0 || cs >= 1.0 {
                return Err("Cache size must be between strictly 0 and 1, otherwise just enable the destinations vector.".to_owned());
            }
            let cached_nodes_number: NodeT = (self.get_nodes_number() as f64 * cs) as NodeT;
            if cached_nodes_number == 0 || cached_nodes_number == self.get_nodes_number() {
                return Err("Required cached nodes number cannot be 0 or all the nodes.".to_owned());
            }
            self.cached_destinations = Some(
                self.get_top_k_central_nodes_ids(cached_nodes_number)
                    .par_iter()
                    .map(|node_id| {
                        (
                            *node_id,
                            self.iter_node_neighbours_ids(*node_id)
                                .collect::<Vec<NodeT>>(),
                        )
                    })
                    .collect::<HashMap<NodeT, Vec<NodeT>>>(),
            );
        } else {
            self.cached_destinations = None;
        }
        Ok(())
    }

    /// Disable all extra perks, reducing memory impact but incresing time requirements.
    pub fn disable_all(&mut self) {
        self.destinations = None;
        self.sources = None;
        self.outbounds = None;
        self.cached_destinations = None;
    }
}

use super::*;

/// Structure that saves the writer specific to writing and reading a nodes csv file.
///
/// # Attributes
/// * writer: CSVFileWriter - The common writer for readin and writing a csv.
/// * nodes_column: String - The name of the nodes names column. This parameter is mutually exclusive with nodes_column_number.
/// * nodes_column_number: usize - The rank of the column with the nodes names. This parameter is mutually exclusive with nodes_column.
/// * node_types_column: String - The name of the nodes type column. This parameter is mutually exclusive with node_types_column_number.
/// * node_types_column_number: usize - The rank of the column with the nodes types. This parameter is mutually exclusive with node_types_column.
/// * node_types_separator: String - Separator to split the node types.
pub struct NodeFileWriter {
    pub(crate) writer: CSVFileWriter,
    pub(crate) nodes_column: String,
    pub(crate) node_types_column: String,
    pub(crate) nodes_column_number: usize,
    pub(crate) node_types_column_number: usize,
    pub(crate) node_types_separator: String,
}

impl NodeFileWriter {
    /// Return new NodeFileWriter object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> NodeFileWriter {
        NodeFileWriter {
            writer: CSVFileWriter::new(path),
            nodes_column: "id".to_string(),
            nodes_column_number: 0,
            node_types_column: "category".to_string(),
            node_types_column_number: 1,
            node_types_separator: "".to_string(),
        }
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_nodes_column<S: Into<String>>(mut self, nodes_column: Option<S>) -> NodeFileWriter {
        if let Some(column) = nodes_column {
            self.nodes_column = column.into();
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_node_types_column<S: Into<String>>(
        mut self,
        nodes_type_column: Option<S>,
    ) -> NodeFileWriter {
        if let Some(column) = nodes_type_column {
            self.node_types_column = column.into();
        }
        self
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column_number: Option<usize> - The nodes column_number to use for the file.
    ///
    pub fn set_nodes_column_number(mut self, nodes_column_number: Option<usize>) -> NodeFileWriter {
        if let Some(column) = nodes_column_number {
            self.nodes_column_number = column;
        }
        self
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column_number: Option<usize> - The node types column_number to use for the file.
    ///
    pub fn set_node_types_column_number(
        mut self,
        node_types_column_number: Option<usize>,
    ) -> NodeFileWriter {
        if let Some(v) = node_types_column_number {
            self.node_types_column_number = v;
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> NodeFileWriter {
        if let Some(v) = verbose {
            self.writer.verbose = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(mut self, separator: Option<S>) -> NodeFileWriter {
        if let Some(v) = separator {
            self.writer.separator = v.into();
        }
        self
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - whether to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> NodeFileWriter {
        if let Some(v) = header {
            self.writer.header = v;
        }
        self
    }

    /// Write nodes to file.
    ///
    /// # Arguments
    ///
    /// * `graph`: &Graph, reference to graph to use.
    pub fn dump(&self, graph: &Graph) -> Result<(), String> {
        // build the header
        let mut header = vec![(self.nodes_column.clone(), self.nodes_column_number)];

        if graph.has_node_types() {
            header.push((
                self.node_types_column.clone(),
                self.node_types_column_number,
            ));
        }

        let number_of_columns = 1 + header.iter().map(|(_, i)| i).max().unwrap();

        self.writer.write_lines(
            graph.get_nodes_number() as usize,
            compose_lines(number_of_columns, header),
            (0..graph.get_nodes_number()).map(|node_id| {
                let mut line = vec![(
                    graph.nodes.unchecked_translate(node_id),
                    self.nodes_column_number,
                )];

                if graph.has_node_types() {
                    line.push((
                        match graph.get_node_type_name_by_node_id(node_id).unwrap() {
                            Some(values) => values.join(&self.node_types_separator),
                            None => "".to_string(),
                        },
                        self.node_types_column_number,
                    ));
                }
                compose_lines(number_of_columns, line)
            }),
        )
    }
}

use super::types::*;
use super::Graph;
use std::cmp::min;
use std::collections::HashSet;

/// # Tarjan algorithm
impl Graph {
    /// Returns list of nodes of the various strongly connected components.
    ///
    /// This is an implementation of Tarjan algorithm.
    ///
    pub fn strongly_connected_components(&self) -> Vec<HashSet<NodeT>> {
        let mut indexed_mask: Vec<bool> = vec![false; self.get_nodes_number() as usize];
        let mut stacked_mask: Vec<bool> = vec![false; self.get_nodes_number() as usize];
        let mut low_indices: Vec<NodeT> = vec![0; self.get_nodes_number() as usize];
        let mut indices: Vec<NodeT> = vec![0; self.get_nodes_number() as usize];
        let mut components_stack: Vec<NodeT> = Vec::new();
        let mut components: Vec<HashSet<NodeT>> = Vec::new();
        let mut common_index = 0;
        let mut recurse: bool;
        for node in 0..self.get_nodes_number() {
            if !indexed_mask[node as usize] {
                let mut to_visit: Vec<(NodeT, usize)> = vec![(node, 0)];
                while !to_visit.is_empty() {
                    let (src, i) = to_visit.pop().unwrap();
                    if !indexed_mask[src as usize] {
                        low_indices[src as usize] = common_index;
                        indices[src as usize] = common_index;
                        indexed_mask[src as usize] = true;
                        stacked_mask[src as usize] = true;
                        common_index += 1;
                        components_stack.push(src);
                    }
                    recurse = false;
                    let (_min, _max) = self.get_minmax_edge_ids_by_source_node_id(src);
                    // Consider successors of source node
                    for (j, dst) in ((_min + i as EdgeT).._max)
                        .map(|edge_id| self.get_unchecked_destination_node_id_by_edge_id(edge_id))
                        .enumerate()
                    {
                        if !indexed_mask[dst as usize] {
                            // Successor w has not yet been visited; recurse on it
                            to_visit.push((src, i + j + 1));
                            to_visit.push((dst, 0));
                            recurse = true;
                            break;
                        } else if stacked_mask[dst as usize] {
                            // Successor w is in stack S and hence in the current SCC
                            // If w is not on stack, then (v, w) is an edge pointing to an SCC already found and must be ignored
                            // Note: The next line may look odd - but is correct.
                            // It says w.index not w.lowlink; that is deliberate and from the original paper
                            low_indices[src as usize] =
                                min(low_indices[src as usize], indices[dst as usize]);
                        }
                    }

                    if recurse {
                        continue;
                    }

                    // If source is a root node, pop the stack and generate an SCC
                    if low_indices[src as usize] == indices[src as usize] {
                        // start a new strongly connected component
                        let mut new_component: HashSet<NodeT> = HashSet::new();
                        loop {
                            let dst = components_stack.pop().unwrap();
                            stacked_mask[dst as usize] = false;
                            new_component.insert(dst);
                            if dst == src {
                                break;
                            }
                        }
                        components.push(new_component);
                    }

                    if !to_visit.is_empty() {
                        let (root, _) = to_visit.last().unwrap();
                        low_indices[*root as usize] =
                            min(low_indices[*root as usize], low_indices[src as usize]);
                    }
                }
            }
        }
        components
    }
}

use super::*;
use rayon::prelude::*;

/// # Queries
/// The naming convection we follow is `get_X_by_Y`.
impl Graph {
    /// Return the src, dst, edge type and weight of a given edge id
    pub fn get_edge_quadruple(
        &self,
        edge_id: EdgeT,
    ) -> (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>) {
        let (src, dst, edge_type) = self.get_edge_triple(edge_id);
        (
            src,
            dst,
            edge_type,
            self.get_unchecked_weight_by_edge_id(edge_id),
        )
    }

    /// Return the src, dst, edge type of a given edge id
    pub fn get_edge_triple(&self, edge_id: EdgeT) -> (NodeT, NodeT, Option<EdgeTypeT>) {
        let (src, dst) = self.get_node_ids_from_edge_id(edge_id);
        (src, dst, self.get_unchecked_edge_type_by_edge_id(edge_id))
    }

    /// Return vector with top k central node Ids.
    ///
    /// # Arguments
    ///
    /// * k: NodeT - Number of central nodes to extract.
    pub fn get_top_k_central_nodes_ids(&self, k: NodeT) -> Vec<NodeT> {
        let mut nodes_degrees: Vec<(NodeT, NodeT)> = (0..self.get_nodes_number())
            .map(|node_id| (self.get_node_degree_by_node_id(node_id).unwrap(), node_id))
            .collect();
        nodes_degrees.par_sort_unstable();
        nodes_degrees.reverse();
        nodes_degrees[0..k as usize]
            .iter()
            .map(|(_, node_id)| *node_id)
            .collect()
    }

    /// Return vector with top k central node names.
    ///
    /// # Arguments
    ///
    /// * k: NodeT - Number of central nodes to extract.
    pub fn get_top_k_central_node_names(&self, k: NodeT) -> Vec<String> {
        self.get_top_k_central_nodes_ids(k)
            .into_iter()
            .map(|node_id| self.get_node_name_by_node_id(node_id).unwrap())
            .collect()
    }

    /// Returns node type of given node.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - node whose node type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The node type id of node {} is {:?}", 0, graph.get_node_type_id_by_node_id(0));
    /// ```
    ///
    pub fn get_node_type_id_by_node_id(
        &self,
        node_id: NodeT,
    ) -> Result<Option<Vec<NodeTypeT>>, String> {
        if let Some(nt) = &self.node_types {
            return if node_id <= nt.ids.len() as NodeT {
                Ok(nt.ids[node_id as usize].clone())
            } else {
                Err(format!(
                    "The node_index {} is too big for the node_types vector which has len {}",
                    node_id,
                    nt.ids.len()
                ))
            };
        }

        Err(String::from(
            "Node types are not defined for current graph instance.",
        ))
    }

    /// Returns edge type of given edge.
    ///
    /// # Arguments
    ///
    /// * edge_id: EdgeT - edge whose edge type is to be returned.
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The edge type id of edge {} is {:?}", 0, graph.get_edge_type_id_by_edge_id(0));
    /// ```
    pub fn get_edge_type_id_by_edge_id(&self, edge_id: EdgeT) -> Result<Option<EdgeTypeT>, String> {
        if let Some(et) = &self.edge_types {
            return if edge_id <= et.ids.len() as EdgeT {
                Ok(self.get_unchecked_edge_type_by_edge_id(edge_id))
            } else {
                Err(format!(
                    "The edge_index {} is too big for the edge_types vector which has len {}",
                    edge_id,
                    et.ids.len()
                ))
            };
        }
        Err(String::from(
            "Edge types are not defined for current graph instance.",
        ))
    }

    /// Returns result of option with the node type of the given node id.
    ///
    /// # Arguments
    /// `node_id`: NodeT - The node ID whose node types are to be returned.
    pub fn get_node_type_name_by_node_id(
        &self,
        node_id: NodeT,
    ) -> Result<Option<Vec<String>>, String> {
        if self.node_types.is_some() {
            Ok(self
                .get_node_type_id_by_node_id(node_id)?
                .and_then(|node_type_ids| {
                    // This unwrap cannot fail because it is surely a vector
                    // of node type IDs from the current graph instance.
                    self.get_node_type_names_by_node_type_ids(node_type_ids)
                        .ok()
                }))
        } else {
            Err("Node types not available for the current graph instance.".to_string())
        }
    }

    /// Returns option with the edge type of the given edge id.
    ///
    /// # Arguments
    /// `edge_id`: EdgeT - The edge ID whose edge type is to be returned.
    pub fn get_edge_type_name_by_edge_id(&self, edge_id: EdgeT) -> Result<Option<String>, String> {
        self.get_edge_type_id_by_edge_id(edge_id)?
            .map_or(Ok(None), |x| {
                Ok(Some(self.get_edge_type_name_by_edge_type_id(x)?))
            })
    }

    /// Return edge type name of given edge type.
    ///
    /// # Arguments
    /// * edge_type_id: EdgeTypeT - Id of the edge type.
    pub fn get_edge_type_name_by_edge_type_id(
        &self,
        edge_type_id: EdgeTypeT,
    ) -> Result<String, String> {
        self.edge_types.as_ref().map_or(
            Err("Edge types not available for the current graph instance.".to_string()),
            |ets| ets.translate(edge_type_id),
        )
    }

    /// Returns weight of the given edge id.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - The edge ID whose weight is to be returned.
    ///
    /// # Examples
    /// To get the weight of a given `edge_id` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let unweighted_graph = graph::test_utilities::load_ppi(true, true, false, true, false, false).unwrap();
    /// let edge_id = 0;
    /// let unexistent_edge_id = 123456789;
    /// assert!(weighted_graph.get_weight_by_edge_id(edge_id).is_ok());
    /// assert!(weighted_graph.get_weight_by_edge_id(unexistent_edge_id).is_err());
    /// assert!(unweighted_graph.get_weight_by_edge_id(edge_id).is_err());
    /// ```
    pub fn get_weight_by_edge_id(&self, edge_id: EdgeT) -> Result<WeightT, String> {
        self.weights.as_ref().map_or(
            Err("The current graph instance does not have weights!".to_string()),
            |weights| weights.get(edge_id as usize).map_or(
                Err(format!(
                    "The given edge_id {} is higher than the number of available directed edges {}.",
                    edge_id,
                    self.get_directed_edges_number()
                )),
                |value| Ok(*value)
            )
        )
    }

    /// Returns weight of the given node ids.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node ID of the source node.
    /// * `dst`: NodeT - The node ID of the destination node.
    ///
    /// # Examples
    /// To get the weight of a given `src` and `dst` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src = 0;
    /// let dst = 1;
    /// assert!(weighted_graph.get_weight_by_node_ids(src, dst).is_ok());
    /// ```
    pub fn get_weight_by_node_ids(&self, src: NodeT, dst: NodeT) -> Result<WeightT, String> {
        self.get_weight_by_edge_id(self.get_edge_id_by_node_ids(src, dst)?)
    }

    /// Returns weight of the given node ids and edge type.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node ID of the source node.
    /// * `dst`: NodeT - The node ID of the destination node.
    /// * `edge_type`: Option<EdgeTypeT> - The edge type ID of the edge.
    ///
    /// # Examples
    /// To get the weight of a given `src` and `dst` and `edge_type` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src = 0;
    /// let dst = 1;
    /// let edge_type = Some(0);
    /// assert!(weighted_graph.get_weight_with_type_by_node_ids(src, dst, edge_type).is_ok());
    /// ```
    pub fn get_weight_with_type_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<WeightT, String> {
        self.get_weight_by_edge_id(self.get_edge_id_with_type_by_node_ids(src, dst, edge_type)?)
    }

    /// Returns weight of the given node names and edge type.
    ///
    /// # Arguments
    /// * `src`: &str - The node name of the source node.
    /// * `dst`: &str - The node name of the destination node.
    /// * `edge_type`: Option<&String> - The edge type name of the edge.
    ///
    /// # Examples
    /// To get the weight of a given `src` and `dst` and `edge_type` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src = "ENSP00000000233";
    /// let dst = "ENSP00000432568";
    /// let edge_type = Some("red".to_string());
    /// assert!(weighted_graph.get_weight_with_type_by_node_names(src, dst, edge_type.as_ref()).is_ok());
    /// ```
    pub fn get_weight_with_type_by_node_names(
        &self,
        src: &str,
        dst: &str,
        edge_type: Option<&String>,
    ) -> Result<WeightT, String> {
        self.get_weight_by_edge_id(self.get_edge_id_with_type_by_node_names(src, dst, edge_type)?)
    }

    /// Returns weight of the given node names.
    ///
    /// # Arguments
    /// * `src_name`: &str - The node name of the source node.
    /// * `dst_name`: &str - The node name of the destination node.
    ///
    /// # Examples
    /// To get the weight of a given `src_name` and `dst_name` you can run:
    /// ```rust
    /// # let weighted_graph = graph::test_utilities::load_ppi(false, true, true, true, false, false).unwrap();
    /// let src_name = "ENSP00000000233";
    /// let dst_name = "ENSP00000432568";
    /// assert!(weighted_graph.get_weight_by_node_names(src_name, dst_name).is_ok());
    /// ```
    pub fn get_weight_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
    ) -> Result<WeightT, String> {
        self.get_weight_by_edge_id(self.get_edge_id_by_node_names(src_name, dst_name)?)
    }

    /// Returns result with the node name.
    pub fn get_node_name_by_node_id(&self, node_id: NodeT) -> Result<String, String> {
        match node_id < self.get_nodes_number() {
            true => Ok(self.nodes.unchecked_translate(node_id)),
            false => Err(format!(
                "Given node_id {} is greater than number of nodes in the graph ({}).",
                node_id,
                self.get_nodes_number()
            )),
        }
    }

    /// Returns result with the node id.
    pub fn get_node_id_by_node_name(&self, node_name: &str) -> Result<NodeT, String> {
        match self.nodes.get(node_name) {
            Some(node_id) => Ok(*node_id),
            None => Err(format!(
                "Given node name {} is not available in current graph.",
                node_name
            )),
        }
    }

    /// Return node type ID for the given node name if available.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Examples
    /// To get the node type ID for a given node name you can run:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// println!("The node type ID of node {} is {:?}.", node_name, graph.get_node_type_id_by_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_type_id_by_node_name(
        &self,
        node_name: &str,
    ) -> Result<Option<Vec<NodeTypeT>>, String> {
        self.get_node_type_id_by_node_id(self.get_node_id_by_node_name(node_name)?)
    }

    /// Return node type name for the given node name if available.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Name of the node.
    ///
    /// # Examples
    /// To get the node type name for a given node name you can run:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// println!("The node type of node {} is {:?}", node_name, graph.get_node_type_name_by_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_type_name_by_node_name(
        &self,
        node_name: &str,
    ) -> Result<Option<Vec<String>>, String> {
        self.get_node_type_name_by_node_id(self.get_node_id_by_node_name(node_name)?)
    }

    /// Return number of edges with given edge type ID.
    ///
    /// If None is given as an edge type ID, the unknown edge type IDs
    /// will be returned.
    ///
    /// # Arguments
    /// edge_type: Option<EdgeTypeT> - The edge type ID to count the edges of.
    ///
    pub fn get_edge_count_by_edge_type_id(
        &self,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<EdgeT, String> {
        if !self.has_edge_types() {
            return Err("Current graph does not have edge types!".to_owned());
        }
        if let Some(et) = &edge_type {
            if self.get_edge_types_number() <= *et {
                return Err(format!(
                    "Given edge type ID {} is bigger than number of edge types in the graph {}.",
                    self.get_edge_types_number(),
                    et
                ));
            }
        }
        Ok(self.get_unchecked_edge_count_by_edge_type_id(edge_type))
    }

    /// Return edge type ID curresponding to given edge type name.
    ///
    /// If None is given as an edge type ID, None is returned.
    ///
    /// # Arguments
    /// edge_type: Option<&str> - The edge type name whose ID is to be returned.
    ///
    pub fn get_edge_type_id_by_edge_type_name(
        &self,
        edge_type_name: Option<&str>,
    ) -> Result<Option<EdgeTypeT>, String> {
        match (&self.edge_types, edge_type_name) {
            (None, _) => Err("Current graph does not have edge types.".to_owned()),
            (Some(_), None) => Ok(None),
            (Some(ets), Some(etn)) => match ets.get(etn) {
                Some(edge_type_id) => Ok(Some(*edge_type_id)),
                None => Err(format!(
                    "Given edge type name {} is not available in current graph.",
                    etn
                )),
            },
        }
    }

    /// Return number of edges with given edge type name.
    ///
    /// If None is given as an edge type name, the unknown edge types
    /// will be returned.
    ///
    /// # Arguments
    /// edge_type: Option<&str> - The edge type name to count the edges of.
    ///
    pub fn get_edge_count_by_edge_type_name(
        &self,
        edge_type: Option<&str>,
    ) -> Result<EdgeT, String> {
        self.get_edge_count_by_edge_type_id(self.get_edge_type_id_by_edge_type_name(edge_type)?)
    }

    /// Return node type ID curresponding to given node type name.
    ///
    /// If None is given as an node type ID, None is returned.
    ///
    /// # Arguments
    /// node_type: Option<&str> - The node type name whose ID is to be returned.
    ///
    pub fn get_node_type_id_by_node_type_name(
        &self,
        node_type_name: &str,
    ) -> Result<NodeTypeT, String> {
        if let Some(ets) = &self.node_types {
            return match ets.get(node_type_name) {
                Some(node_type_id) => Ok(*node_type_id),
                None => Err(format!(
                    "Given node type name {} is not available in current graph.",
                    node_type_name
                )),
            };
        }
        Err("Current graph does not have node types.".to_owned())
    }

    /// Return number of nodes with given node type ID.
    ///
    /// If None is given as an node type ID, the unknown node types
    /// will be returned.
    ///
    /// # Arguments
    /// node_type: Option<NodeTypeT> - The node type ID to count the nodes of.
    ///
    pub fn get_node_count_by_node_type_id(
        &self,
        node_type: Option<NodeTypeT>,
    ) -> Result<NodeT, String> {
        if !self.has_node_types() {
            return Err("Current graph does not have node types!".to_owned());
        }
        if node_type.map_or(false, |nt| self.get_node_types_number() <= nt) {
            return Err(format!(
                "Given node type ID {:?} is bigger than number of node types in the graph {}.",
                node_type,
                self.get_node_types_number()
            ));
        }
        Ok(self.get_unchecked_node_count_by_node_type_id(node_type))
    }

    /// Return number of nodes with given node type name.
    ///
    /// If None is given as an node type name, the unknown node types
    /// will be returned.
    ///
    /// # Arguments
    /// node_type: Option<&str> - The node type name to count the nodes of.
    ///
    pub fn get_node_count_by_node_type_name(
        &self,
        node_type_name: Option<&str>,
    ) -> Result<NodeT, String> {
        self.get_node_count_by_node_type_id(
            node_type_name.map_or(Ok::<_, String>(None), |ntn| {
                Ok(Some(self.get_node_type_id_by_node_type_name(ntn)?))
            })?,
        )
    }

    /// Returns the destination of given edge id without making any boundary check.
    ///
    /// # Arguments
    ///
    /// `edge_id`: EdgeT - The edge ID whose destination is to be retrieved.
    pub(crate) fn get_unchecked_destination_node_id_by_edge_id(&self, edge_id: EdgeT) -> NodeT {
        self.destinations.as_ref().map_or_else(
            || self.get_node_ids_from_edge_id(edge_id).1,
            |dsts| dsts[edge_id as usize],
        )
    }

    /// Returns the destination of given edge id.
    ///
    /// # Arguments
    ///
    /// `edge_id`: EdgeT - The edge ID whose destination is to be retrieved.
    pub fn get_destination_node_id_by_edge_id(&self, edge_id: EdgeT) -> Result<NodeT, String> {
        if edge_id >= self.get_directed_edges_number() {
            return Err(format!(
                "The edge ID {} is higher than the number of available directed edges {}.",
                edge_id,
                self.get_directed_edges_number()
            ));
        }
        Ok(self.get_unchecked_destination_node_id_by_edge_id(edge_id))
    }

    /// Return vector of destinations for the given source node ID.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Node ID whose neighbours are to be retrieved.
    ///
    /// # Example
    /// To retrieve the neighbours of a given node `src` you can use:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// # let node_id = 0;
    /// println!("The neighbours of the node {} are {:?}.", node_id, graph.get_node_neighbours_by_node_id(node_id).unwrap());
    /// let unavailable_node = 2349765432;
    /// assert!(graph.get_node_neighbours_by_node_id(unavailable_node).is_err());
    /// ```
    pub fn get_node_neighbours_by_node_id(&self, node_id: NodeT) -> Result<Vec<NodeT>, String> {
        if node_id >= self.get_nodes_number() {
            return Err(format!(
                "The node ID {} is higher than the number of available nodes {}.",
                node_id,
                self.get_nodes_number()
            ));
        }
        Ok(self.iter_node_neighbours_ids(node_id).collect())
    }

    /// Return vector of destinations for the given source node name.
    ///
    /// # Arguments
    ///
    /// * `node_name`: &str - Node ID whose neighbours are to be retrieved.
    ///
    /// # Example
    /// To retrieve the neighbours of a given node `src` you can use:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// println!("The neighbours of the node {} are {:?}.", node_name, graph.get_node_neighbour_ids_by_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_neighbour_ids_by_node_name(&self, node_name: &str) -> Result<Vec<NodeT>, String> {
        self.get_node_neighbours_by_node_id(self.get_node_id_by_node_name(node_name)?)
    }

    /// Return vector of destination names for the given source node name.
    ///
    /// # Arguments
    ///
    /// * `node_id`: NodeT - Node ID whose neighbours are to be retrieved.
    ///
    /// # Example
    /// To retrieve the neighbours of a given node `src` you can use:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let node_name = "ENSP00000000233";
    /// println!("The neighbours of the node {} are {:?}.", node_name, graph.get_node_neighbour_names_by_node_name(node_name).unwrap());
    /// ```
    pub fn get_node_neighbour_names_by_node_name(
        &self,
        node_name: &str,
    ) -> Result<Vec<String>, String> {
        Ok(self
            .iter_node_neighbours(self.get_node_id_by_node_name(node_name)?)
            .collect())
    }

    /// Return edge ID for given tuple of nodes and edge type.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// # Arguments
    /// `src`: NodeT - Source node of the edge.
    /// `dst`: NodeT - Destination node of the edge.
    /// `edge_type`: Option<EdgeTypeT> - Edge Type of the edge.
    ///
    pub fn get_edge_id_with_type_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_type: Option<EdgeTypeT>,
    ) -> Result<EdgeT, String> {
        self.edge_types
            .as_ref()
            .map_or_else(
                || self.get_edge_id_by_node_ids(src, dst).ok(),
                |ets| {
                    self.iter_edge_ids_by_node_ids(src, dst)
                        .and_then(|mut edge_ids| {
                            edge_ids.find(|edge_id| ets.ids[*edge_id as usize] == edge_type)
                        })
                },
            )
            .ok_or_else(|| {
                format!(
                    concat!(
                    "The current graph instance does not contain the required edge composed of ",
                    "source node ID {}, destination node ID {} and edge ID {:?}."
                ),
                    src, dst, edge_type
                )
            })
    }

    /// Return edge ID for given tuple of node names.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// # Arguments
    /// `src_name`: &str - Source node name of the edge.
    /// `dst_name`: &str - Destination node name of the edge.
    ///
    pub fn get_edge_id_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
    ) -> Result<EdgeT, String> {
        match (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            (Some(src), Some(dst)) => self.get_edge_id_by_node_ids(*src, *dst).ok(),
            _ => None,
        }
        .ok_or_else(|| {
            format!(
                concat!(
                    "The current graph instance does not contain the required edge composed of ",
                    "source node name {} and destination node name {}."
                ),
                src_name, dst_name
            )
        })
    }

    /// Return edge ID for given tuple of node names and edge type name.
    ///
    /// This method will return an error if the graph does not contain the
    /// requested edge with edge type.
    ///
    /// # Arguments
    /// `src_name`: &str - Source node name of the edge.
    /// `dst_name`: &str - Destination node name of the edge.
    /// `edge_type_name`: Option<&String> - Edge type name.
    ///
    pub fn get_edge_id_with_type_by_node_names(
        &self,
        src_name: &str,
        dst_name: &str,
        edge_type_name: Option<&String>,
    ) -> Result<EdgeT, String> {
        match (self.nodes.get(src_name), self.nodes.get(dst_name)) {
            (Some(src), Some(dst)) => self
                .get_edge_id_with_type_by_node_ids(
                    *src,
                    *dst,
                    self.get_edge_type_id_by_edge_type_name(edge_type_name.map(|x| x.as_str()))?,
                )
                .ok(),
            _ => None,
        }
        .ok_or_else(|| {
            format!(
                concat!(
                    "The current graph instance does not contain the required edge composed of ",
                    "source node name {}, destination node name {} and edge name {:?}."
                ),
                src_name, dst_name, edge_type_name
            )
        })
    }

    /// Return translated edge types from string to internal edge ID.
    ///
    /// # Arguments
    ///
    /// * `edge_types`: Vec<String> - Vector of edge types to be converted.
    pub fn get_edge_type_ids_by_edge_type_names(
        &self,
        edge_types: Vec<Option<String>>,
    ) -> Result<Vec<Option<EdgeTypeT>>, String> {
        match &self.edge_types {
                None => Err(String::from("Current graph does not have edge types.")),
                Some(ets) => {
                    edge_types
                    .iter()
                    .map(|edge_type_name|
                        match edge_type_name {
                            None=> Ok(None),
                            Some(et) => {
                                match ets.get(et) {
                                    Some(edge_type_id) => Ok(Some(*edge_type_id)),
                                    None => Err(format!(
                                        "The edge type {} does not exist in current graph. The available edge types are {}.",
                                        et,
                                        ets.keys().join(", ")
                                    ))
                                }
                            }
                        }
                    )
                .collect::<Result<Vec<Option<EdgeTypeT>>, String>>()
            }
        }
    }

    /// Return translated node types from string to internal node ID.
    ///
    /// # Arguments
    ///
    /// * `node_types`: Vec<String> - Vector of node types to be converted.
    pub fn get_node_type_ids_by_node_type_names(
        &self,
        node_types: Vec<Option<String>>,
    ) -> Result<Vec<Option<NodeTypeT>>, String> {
        match &self.node_types {
            None => Err(String::from("Current graph does not have node types.")),
            Some(nts) => {
                node_types
                .iter()
                .map(|node_type_name|
                    match node_type_name {
                        None => Ok(None),
                        Some(nt) => {
                            match nts.get(nt) {
                                Some(node_type_id) => Ok(Some(*node_type_id)),
                                None => Err(format!(
                                    "The node type {} does not exist in current graph. The available node types are {}.",
                                    nt,
                                    nts.keys().join(", ")
                                )),
                            }
                        }
                    })
                .collect::<Result<Vec<Option<NodeTypeT>>, String>>()
            }
        }
    }

    /// Return range of outbound edges IDs for all the edges bewteen the given
    /// source and destination nodes.
    /// This operation is meaningfull only in a multigraph.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Source node.
    /// * dst: NodeT - Destination node.
    ///
    pub(crate) fn get_minmax_edge_ids_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Option<(EdgeT, EdgeT)> {
        self.get_edge_id_by_node_ids(src, dst).ok().map(|min_edge| {
            (
                min_edge,
                self.get_unchecked_edge_id_from_tuple(src, dst + 1),
            )
        })
    }

    /// Return range of outbound edges IDs which have as source the given Node.
    ///
    /// # Arguments
    ///
    /// * src: NodeT - Node for which we need to compute the outbounds range.
    ///
    pub(crate) fn get_minmax_edge_ids_by_source_node_id(&self, src: NodeT) -> (EdgeT, EdgeT) {
        match &self.outbounds {
            Some(outbounds) => {
                let min_edge_id = if src == 0 {
                    0
                } else {
                    outbounds[src as usize - 1]
                };
                (min_edge_id, outbounds[src as usize])
            }
            None => {
                let min_edge_id: EdgeT = self.get_unchecked_edge_id_from_tuple(src, 0);
                (
                    min_edge_id,
                    match &self.cached_destinations {
                        Some(cds) => match cds.get(&src) {
                            Some(destinations) => destinations.len() as EdgeT + min_edge_id,
                            None => self.get_unchecked_edge_id_from_tuple(src + 1, 0),
                        },
                        None => self.get_unchecked_edge_id_from_tuple(src + 1, 0),
                    },
                )
            }
        }
    }

    /// Return node type name of given node type.
    ///
    /// There is no need for a unchecked version since we will have to map
    /// on the note_types anyway.
    ///
    /// # Arguments
    /// * node_type_id: Vec<NodeTypeT> - Id of the node type.
    pub fn get_node_type_name_by_node_type_id(
        &self,
        node_type_id: NodeTypeT,
    ) -> Result<String, String> {
        self.node_types.as_ref().map_or(
            Err("Node types not available for the current graph instance.".to_string()),
            |nts| nts.translate(node_type_id),
        )
    }

    /// Return node type name of given node type.
    ///
    /// # Arguments
    /// * node_type_ids: Vec<NodeTypeT> - Id of the node type.
    pub fn get_node_type_names_by_node_type_ids(
        &self,
        node_type_ids: Vec<NodeTypeT>,
    ) -> Result<Vec<String>, String> {
        self.node_types.as_ref().map_or(
            Err("Node types not available for the current graph instance.".to_string()),
            |nts| nts.translate_vector(node_type_ids),
        )
    }

    /// Returns the number of outbound neighbours of given node.
    ///
    /// This is implemented as proposed by [S. Vigna here](http://vigna.di.unimi.it/ftp/papers/Broadword.pdf).
    ///
    /// # Arguments
    ///
    /// * `node_id` - Integer ID of the node.
    ///
    pub fn get_node_degree_by_node_id(&self, node_id: NodeT) -> Result<NodeT, String> {
        if node_id >= self.get_nodes_number() {
            return Err(format!(
                "The node ID {} is higher than the number of available nodes {}.",
                node_id,
                self.get_nodes_number()
            ));
        }
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_by_source_node_id(node_id);
        Ok((max_edge_id - min_edge_id) as NodeT)
    }
}

use super::*;
use indicatif::ProgressIterator;

impl Graph {
    /// Return graph filtered by given weights range.
    ///
    /// # Arguments
    ///
    /// * node_names: Option<Vec<String>> - The node names to keep.
    /// * node_types: Option<Vec<String>> - The node types to keep.
    /// * edge_types: Option<Vec<String>> - The edge types to keep.
    /// * min_weight: Option<WeightT> - Minimum weight to use to filter edges.
    /// * max_weight: Option<WeightT> - Maximum weight to use to filter edges.
    /// * verbose: bool - whether to show the loading bar.
    ///
    pub fn filter(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
        edge_types: Option<Vec<Option<String>>>,
        min_weight: Option<WeightT>,
        max_weight: Option<WeightT>,
        verbose: bool,
    ) -> Result<Graph, String> {
        if let (Some(min_w), Some(max_w)) = (min_weight, max_weight) {
            if min_w >= max_w {
                return Err(format!(
                    "The given minimum weight ({}) is greater or equal than the given maximum weight ({})!",
                    min_w, max_w
                ));
            }
        }

        let pb = get_loading_bar(
            verbose,
            format!("Building filtered {}", self.name).as_ref(),
            self.get_directed_edges_number() as usize,
        );

        let node_ids = self.get_filter_bitmap(node_names, node_types)?;
        let edge_types_ids = edge_types.map_or(Ok::<_, String>(None), |ets| {
            Ok(Some(self.get_edge_type_ids_by_edge_type_names(ets)?))
        });
        let edge_types_ids = edge_types_ids?;

        Graph::build_graph(
            self.iter_edge_with_type_and_weight_ids(true)
                .progress_with(pb)
                .filter_map(|(_, src, dst, edge_type, weight)| {
                    if let Some(nis) = &node_ids {
                        if !nis.contains(src) || !nis.contains(dst) {
                            return None;
                        }
                    }
                    if let (Some(_min), Some(w)) = (min_weight, weight) {
                        if _min > w {
                            return None;
                        }
                    }
                    if let (Some(_max), Some(w)) = (max_weight, weight) {
                        if w >= _max {
                            return None;
                        }
                    }
                    if let Some(ets) = &edge_types_ids {
                        if !ets.contains(&edge_type) {
                            return None;
                        }
                    }
                    Some(Ok((src, dst, edge_type, weight)))
                }),
            self.get_directed_edges_number() as usize,
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.directed,
            true,
            self.name.clone(),
            false,
            self.has_edge_types(),
            self.has_weights(),
            true,
            true,
            true,
        )
    }
}

use std::fmt::Display;
use std::hash::Hash;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::ops::AddAssign;

// Types used to represent edges, nodes and their types.
/// Type used to index the Nodes.
pub type NodeT = u32;
/// Type used to index the Node Types.
pub type NodeTypeT = u16;
/// Type used to index the Edges.
pub type EdgeT = u64;
/// Type used to index the Edge Types.
pub type EdgeTypeT = u16;
/// Type used for the weights of the edges.
pub type WeightT = f32;
/// Type used for the parameters of the walk such as the return weight (p),
/// and the explore weight (q).
pub type ParamsT = WeightT;
/// Type used to save contexts used for Skipgram and CBOW.
pub type Contexts = Vec<Vec<NodeT>>;
/// Type used to save a group of words indices.
pub type Words = Vec<NodeT>;
/// Type used to save the frequencies of words
pub type Frequencies = Vec<f64>;
/// Triple of edge data
pub type Triple = (NodeT, NodeT, Option<EdgeTypeT>);
/// Quadruple of edge data
pub type Quadruple = (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>);
/// Quadrule of string edge data
pub type StringQuadruple = (String, String, Option<String>, Option<WeightT>);

/// Trait used for the Vocabulary class.
/// It represent an unsigned integer that can be converted to and from usize.
/// This allows us to save memory using indicies of smaller size than u64
/// and it has no effects on performance because it's optimized away during
/// compilaton.
pub trait ToFromUsize: Clone + Display + Ord + Copy + AddAssign + Hash {
    /// create the type from a usize
    fn from_usize(v: usize) -> Self;
    /// create an usize frm the type
    fn to_usize(v: Self) -> usize;
}

/// Automatically implement the methods needed to convert from and to usize
/// for the given numerical type.
macro_rules! impl_to_from_usize {
    ($($ty:ty)*) => {
        $(
            impl ToFromUsize for $ty {
                #[inline(always)]
                fn from_usize(v: usize) -> $ty {
                    v as $ty
                }
                #[inline(always)]
                fn to_usize(v: $ty) -> usize {
                    v as usize
                }
            }
        )*
    }
}

impl_to_from_usize!(u8 u16 u32 u64 usize);

#[derive(Debug)]
pub(crate) struct ClonableRwLock<T: Clone + std::fmt::Debug> {
    value: RwLock<T>
}

impl<T: Clone + std::fmt::Debug> ClonableRwLock<T> {
    pub fn new(val: T) -> ClonableRwLock<T> {
        ClonableRwLock{
            value: RwLock::new(val)
        }
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.value.read().unwrap()
    }
    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.value.write().unwrap()
    }
}

impl<T: Clone + std::fmt::Debug> Clone for ClonableRwLock<T>{
    fn clone(&self) -> ClonableRwLock<T> {
        ClonableRwLock{
            value: RwLock::new(self.read().clone())
        }
    }
}
use super::types::*;
use super::*;
use itertools::Itertools;
use log::info;
use rayon::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap as DefaultHashMap;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// # Properties and measurements of the graph
impl Graph {
    /// Returns product of degrees of given nodes.
    ///
    /// # Arguments
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The degrees_product between 0 and 1 is {}", graph.degrees_product(0, 1).unwrap());
    /// ```
    pub fn degrees_product(&self, one: NodeT, two: NodeT) -> Result<usize, String> {
        if one >= self.get_nodes_number() || two >= self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "One or more of the given nodes indices ({}, {}) are ",
                    "biggen than the number of nodes present in the graph ({})."
                ),
                one,
                two,
                self.get_nodes_number()
            ));
        }
        Ok(self.get_node_degree_by_node_id(one).unwrap() as usize
            * self.get_node_degree_by_node_id(two).unwrap() as usize)
    }

    /// Returns the Jaccard index for the two given nodes.
    ///
    /// # Arguments
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The Jaccard Index between node 1 and node 2 is {}", graph.jaccard_index(1, 2).unwrap());
    /// ```
    pub fn jaccard_index(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        if one >= self.get_nodes_number() || two >= self.get_nodes_number() {
            return Err(format!(
                concat!(
                    "One or more of the given nodes indices ({}, {}) are ",
                    "biggen than the number of nodes present in the graph ({})."
                ),
                one,
                two,
                self.get_nodes_number()
            ));
        }

        if self.is_node_trap_by_node_id(one).unwrap() || self.is_node_trap_by_node_id(two).unwrap()
        {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.iter_node_neighbours_ids(one).collect();
        let two_neighbors: HashSet<NodeT> = self.iter_node_neighbours_ids(two).collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        Ok(intersections.len() as f64 / (one_neighbors.len() + two_neighbors.len()) as f64)
    }

    /// Returns the Adamic/Adar Index for the given pair of nodes.
    ///
    /// # Arguments:
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    /// # Implementation details
    /// Since the Adamic/Adar Index is only defined for graph not containing
    /// node traps (nodes without any outbound edge) and must support all kind
    /// of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    /// # References
    /// [D. Liben-Nowell, J. Kleinberg.
    /// The Link Prediction Problem for Social Networks (2004).](http://www.cs.cornell.edu/home/kleinber/link-pred.pdf)
    ///
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The Adamic/Adar Index between node 1 and node 2 is {}", graph.adamic_adar_index(1, 2).unwrap());
    /// ```
    pub fn adamic_adar_index(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        if self.is_node_trap_by_node_id(one)? || self.is_node_trap_by_node_id(two)? {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.iter_node_neighbours_ids(one).collect();
        let two_neighbors: HashSet<NodeT> = self.iter_node_neighbours_ids(two).collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        Ok(intersections
            .par_iter()
            .filter(|node| !self.is_node_trap_by_node_id(**node).unwrap())
            .map(|node| 1.0 / (self.get_node_degree_by_node_id(*node).unwrap() as f64).ln())
            .sum())
    }

    /// Returns the Resource Allocation Index for the given pair of nodes.
    ///
    /// # Arguments:
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    /// # References
    /// [T. Zhou, L. Lu, Y.-C. Zhang.
    /// Predicting missing links via local information.
    /// Eur. Phys. J. B 71 (2009) 623.](http://arxiv.org/pdf/0901.0553.pdf)
    ///
    /// # Implementation details
    /// Since the Resource Allocation Index is only defined for graph not
    /// containing node traps (nodes without any outbound edge) and
    /// must support all kind of graphs, the sinks node are excluded from
    /// the computation because they would result in an infinity.
    ///
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The Resource Allocation Index between node 1 and node 2 is {}", graph.resource_allocation_index(1, 2).unwrap());
    /// ```
    pub fn resource_allocation_index(&self, one: NodeT, two: NodeT) -> Result<f64, String> {
        if self.is_node_trap_by_node_id(one)? || self.is_node_trap_by_node_id(two)? {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.iter_node_neighbours_ids(one).collect();
        let two_neighbors: HashSet<NodeT> = self.iter_node_neighbours_ids(two).collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        Ok(intersections
            .par_iter()
            .filter(|node| !self.is_node_trap_by_node_id(**node).unwrap())
            .map(|node| 1.0 / self.get_node_degree_by_node_id(*node).unwrap() as f64)
            .sum())
    }

    /// Returns the traps rate of the graph.
    ///
    /// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    ///
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The Graph rate is {}", graph.traps_rate());
    /// ```
    pub fn traps_rate(&self) -> f64 {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| {
                if !self.is_node_trap_by_node_id(node).unwrap() {
                    self.iter_node_neighbours_ids(node)
                        .map(|dst| self.is_node_trap_by_node_id(dst).unwrap() as usize as f64)
                        .sum::<f64>()
                        / self.get_node_degree_by_node_id(node).unwrap() as f64
                } else {
                    1.0
                }
            })
            .sum::<f64>()
            / self.get_nodes_number() as f64
    }

    /// Returns mean node degree of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The mean node degree of the graph is  {}", graph.get_node_degrees_mean().unwrap());
    /// ```
    pub fn get_node_degrees_mean(&self) -> Result<f64, String> {
        if !self.has_nodes() {
            return Err(
                "The mean of the node degrees is not defined on an empty graph".to_string(),
            );
        }
        Ok(self.get_directed_edges_number() as f64 / self.get_nodes_number() as f64)
    }

    /// Returns number of undirected edges of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of undirected edges of the graph is  {}", graph.get_undirected_edges_number());
    /// ```
    pub fn get_undirected_edges_number(&self) -> EdgeT {
        (self.get_directed_edges_number() - self.get_self_loop_number()) / 2
            + self.get_self_loop_number()
    }

    /// Returns number of undirected edges of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of unique undirected edges of the graph is  {}", graph.get_unique_undirected_edges_number());
    /// ```
    pub fn get_unique_undirected_edges_number(&self) -> EdgeT {
        (self.unique_edges_number - self.get_unique_self_loop_number() as EdgeT) / 2
            + self.get_unique_self_loop_number() as EdgeT
    }

    /// Returns number of edges of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of edges of the graph is  {}", graph.get_edges_number());
    /// ```
    pub fn get_edges_number(&self) -> EdgeT {
        match self.directed {
            true => self.get_directed_edges_number(),
            false => self.get_undirected_edges_number(),
        }
    }

    /// Returns number of unique edges of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of edges of the graph is  {}", graph.get_unique_edges_number());
    /// ```
    pub fn get_unique_edges_number(&self) -> EdgeT {
        match self.directed {
            true => self.get_unique_directed_edges_number(),
            false => self.get_unique_undirected_edges_number(),
        }
    }

    /// Returns median node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The median node degree of the graph is  {}", graph.get_node_degrees_median().unwrap());
    /// ```
    pub fn get_node_degrees_median(&self) -> Result<NodeT, String> {
        if !self.has_nodes() {
            return Err(
                "The median of the node degrees is not defined on an empty graph".to_string(),
            );
        }
        let mut degrees = self.get_node_degrees();
        degrees.par_sort_unstable();
        Ok(degrees[(self.get_nodes_number() / 2) as usize])
    }

    /// Returns maximum node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The maximum node degree of the graph is  {}", graph.get_max_node_degree().unwrap());
    /// ```
    pub fn get_max_node_degree(&self) -> Result<NodeT, String> {
        self.get_node_degrees().into_iter().max().ok_or_else(|| {
            "The maximum node degree of a graph with no nodes is not defined.".to_string()
        })
    }

    /// Returns minimum node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The minimum node degree of the graph is  {}", graph.get_min_node_degree().unwrap());
    /// ```
    pub fn get_min_node_degree(&self) -> Result<NodeT, String> {
        self.get_node_degrees().into_iter().min().ok_or_else(|| {
            "The minimum node degree of a graph with no nodes is not defined.".to_string()
        })
    }

    /// Returns mode node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The mode node degree of the graph is  {}", graph.get_node_degrees_mode().unwrap());
    /// ```
    pub fn get_node_degrees_mode(&self) -> Result<NodeT, String> {
        if !self.has_nodes() {
            return Err(
                "The mode of the node degrees is not defined on an empty graph".to_string(),
            );
        }

        let mut occurrences: HashMap<NodeT, usize> = HashMap::new();

        for value in self.get_node_degrees() {
            *occurrences.entry(value).or_insert(0) += 1;
        }
        Ok(occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .unwrap())
    }

    /// Returns number of self-loops, including also those in eventual multi-edges.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of self-loops in the graph is  {}", graph.get_self_loop_number());
    /// ```
    pub fn get_self_loop_number(&self) -> EdgeT {
        self.self_loop_number
    }

    /// Returns number of unique self-loops, excluding those in eventual multi-edges.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of unique self-loops in the graph is  {}", graph.get_unique_self_loop_number());
    /// ```
    pub fn get_unique_self_loop_number(&self) -> NodeT {
        self.unique_self_loop_number
    }

    /// Returns rate of self-loops.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The rate of self-loops in the graph is  {}", graph.get_self_loop_rate().unwrap());
    /// ```
    pub fn get_self_loop_rate(&self) -> Result<f64, String> {
        if !self.has_edges() {
            return Err("The self-loops rate is not defined for graphs without edges.".to_string());
        }
        Ok(self.get_self_loop_number() as f64 / self.get_directed_edges_number() as f64)
    }

    /// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
    pub fn connected_components_number(&self, verbose: bool) -> (NodeT, NodeT, NodeT) {
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
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph contains {} singleton nodes", graph.get_singleton_nodes_number());
    /// ```
    pub fn get_singleton_nodes_number(&self) -> NodeT {
        self.get_nodes_number() - self.get_not_singleton_nodes_number()
    }

    /// Returns number of singleton nodes with self-loops within the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph contains {} singleton nodes with self-loops", graph.get_singleton_nodes_with_self_loops_number());
    /// ```
    pub fn get_singleton_nodes_with_self_loops_number(&self) -> NodeT {
        self.singleton_nodes_with_self_loops_number
    }

    /// Returns number of not singleton nodes within the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph contains {} not singleton nodes", graph.get_not_singleton_nodes_number());
    /// ```
    pub fn get_not_singleton_nodes_number(&self) -> NodeT {
        self.not_singleton_nodes_number
    }

    /// Returns density of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph density is {}", graph.get_density().unwrap());
    /// ```
    pub fn get_density(&self) -> Result<f64, String> {
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

    /// Returns report relative to the graph metrics
    ///
    /// The report includes a few useful metrics like:
    ///
    /// * degrees_median: the median degree of the nodes.
    /// * degrees_mean: the mean degree of the nodes.
    /// * degrees_mode: the mode degree of the nodes.
    /// * min_degree: the max degree of the nodes.
    /// * max_degree: the min degree of the nodes.
    /// * nodes_number: the number of nodes in the graph.
    /// * edges_number: the number of edges in the graph.
    /// * unique_node_types_number: the number of different node types in the graph.
    /// * unique_edge_types_number: the number of different edge types in the graph.
    /// * traps_rate: probability to end up in a trap when starting into any given node.
    /// * selfloops_rate: pecentage of edges that are selfloops.
    /// * bidirectional_rate: rate of edges that are bidirectional.
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// graph.report();
    /// ```
    pub fn report(&self) -> DefaultHashMap<&str, String> {
        let mut report: DefaultHashMap<&str, String> = DefaultHashMap::new();

        if self.has_nodes() {
            report.insert("density", self.get_density().unwrap().to_string());
            report.insert(
                "min_degree",
                self.get_min_node_degree().unwrap().to_string(),
            );
            report.insert(
                "max_degree",
                self.get_max_node_degree().unwrap().to_string(),
            );
            report.insert(
                "degree_mean",
                self.get_node_degrees_mean().unwrap().to_string(),
            );
        }

        if self.has_edges() {
            report.insert(
                "self_loops_rate",
                self.get_self_loop_rate().unwrap().to_string(),
            );
        }

        report.insert("name", self.name.clone());
        report.insert("nodes_number", self.get_nodes_number().to_string());
        report.insert("edges_number", self.get_directed_edges_number().to_string());
        report.insert(
            "undirected_edges_number",
            self.get_undirected_edges_number().to_string(),
        );
        report.insert("directed", self.is_directed().to_string());
        report.insert("has_weights", self.has_weights().to_string());
        report.insert("has_edge_types", self.has_edge_types().to_string());
        report.insert("has_node_types", self.has_node_types().to_string());
        report.insert("self_loops_number", self.get_self_loop_number().to_string());
        report.insert("singletons", self.get_singleton_nodes_number().to_string());
        report.insert(
            "unique_node_types_number",
            self.get_node_types_number().to_string(),
        );
        report.insert(
            "unique_edge_types_number",
            self.get_edge_types_number().to_string(),
        );
        report
    }

    fn shared_components_number(&self, nodes_components: &[NodeT], other: &Graph) -> NodeT {
        other
            .iter_nodes()
            .filter_map(
                |(_, node_name, _, _)| match self.get_node_id_by_node_name(&node_name) {
                    Ok(node_id) => Some(nodes_components[node_id as usize]),
                    Err(_) => None,
                },
            )
            .unique()
            .count() as NodeT
    }

    /// Return number of distinct components that are merged by the other graph in current graph.bitvec
    ///
    /// # Arguments
    /// * `nodes_components`: &[NodeT] - Slice with the node components.
    /// * `other`: &Graph - Graph from where to extract the edge list.
    fn merged_components_number(&self, nodes_components: &[NodeT], other: &Graph) -> NodeT {
        other
            .iter_edges(false)
            .filter_map(|(_, _, src_name, _, dst_name)| {
                match (
                    self.get_node_id_by_node_name(&src_name),
                    self.get_node_id_by_node_name(&dst_name),
                ) {
                    (Ok(src_id), Ok(dst_id)) => {
                        let src_component_number = nodes_components[src_id as usize];
                        let dst_component_number = nodes_components[dst_id as usize];
                        match src_component_number == dst_component_number {
                            true => None,
                            false => Some(vec![src_component_number, dst_component_number]),
                        }
                    }
                    _ => None,
                }
            })
            .flatten()
            .unique()
            .count() as NodeT
    }

    /// Return rendered textual report about the graph overlaps.
    ///
    /// # Arguments
    ///
    /// - `other`: &Graph - graph to create overlap report with.
    /// - `verbose`: bool - whether to shor the loading bars.
    pub fn overlap_textual_report(&self, other: &Graph, verbose: bool) -> Result<String, String> {
        // Checking if overlap is allowed
        self.validate_operator_terms(other)?;
        // Get overlapping nodes
        let overlapping_nodes_number = self
            .iter_nodes()
            .filter(|(_, node_name, _, node_type)| {
                other.has_node_with_type_by_node_name(node_name, node_type.clone())
            })
            .count();
        // Get overlapping edges
        let overlapping_edges_number = self
            .par_iter_edge_with_type(self.directed)
            .filter(|(_, _, src_name, _, dst_name, _, edge_type_name)| {
                other.has_edge_with_type_by_node_names(src_name, dst_name, edge_type_name.as_ref())
            })
            .count();
        // Get number of overlapping components
        let first_nodes_components = self.get_node_components_vector(verbose);
        let second_nodes_components = other.get_node_components_vector(verbose);
        let first_components_number = first_nodes_components.iter().unique().count() as NodeT;
        let second_components_number = second_nodes_components.iter().unique().count() as NodeT;
        let first_shared_components_number =
            self.shared_components_number(&first_nodes_components, other);
        let second_shared_components_number =
            other.shared_components_number(&second_nodes_components, self);
        // Get number of overlapping components
        let first_merged_components_number =
            self.merged_components_number(&first_nodes_components, other);
        let second_merged_components_number =
            other.merged_components_number(&second_nodes_components, self);

        let first_edges = match self.directed {
            true => self.get_directed_edges_number(),
            false => self.get_undirected_edges_number(),
        };
        let second_edges = match other.directed {
            true => other.get_directed_edges_number(),
            false => other.get_undirected_edges_number(),
        };
        // Building up the report
        Ok(format!(
            concat!(
                "The graph {first_graph} and the graph {second_graph} share {nodes_number} nodes and {edges_number} edges. ",
                "By percent, {first_graph} shares {first_node_percentage:.2}% ({nodes_number} out of {first_nodes}) of its nodes and {first_edge_percentage:.2}% ({edges_number} out of {first_edges}) of its edges with {second_graph}. ",
                "{second_graph} shares {second_node_percentage:.2}% ({nodes_number} out of {second_nodes}) of its nodes and {second_edge_percentage:.2}% ({edges_number} out of {second_edges}) of its edges with {first_graph}. ",
                "Nodes from {first_graph} appear in {first_components_statement} components of {second_graph}{first_merged_components_statement}. ",
                "Similarly, nodes from {second_graph} appear in {second_components_statement} components of {first_graph}{second_merged_components_statement}. ",
            ),
            first_graph=self.get_name(),
            second_graph=other.get_name(),
            nodes_number=overlapping_nodes_number,
            edges_number=overlapping_edges_number,
            first_nodes=self.get_nodes_number(),
            second_nodes=other.get_nodes_number(),
            first_edges=first_edges,
            second_edges=second_edges,
            first_components_statement = match second_shared_components_number== second_components_number{
                true=> "all the".to_owned(),
                false => format!(
                    "{second_shared_components_number} of the {second_components_number}",
                    second_shared_components_number=second_shared_components_number,
                    second_components_number=second_components_number
                )
            },
            second_components_statement = match first_shared_components_number== first_components_number{
                true=> "all the".to_owned(),
                false => format!(
                    "{first_shared_components_number} of the {first_components_number}",
                    first_shared_components_number=first_shared_components_number,
                    first_components_number=first_components_number
                )
            },
            first_merged_components_statement = match second_components_number > 1 {
                false=>"".to_owned(),
                true=>format!(
                    ": of these, {edges_number} connected by edges of {first_graph}",
                    first_graph=self.name,
                    edges_number= match second_merged_components_number {
                        d if d==0=>"none are".to_owned(),
                        d if d==1=>"one is".to_owned(),
                        d if d==second_components_number=>"all components are".to_owned(),
                        _ => format!("{} components are", second_merged_components_number)
                    })
                },
            second_merged_components_statement = match first_components_number > 1 {
                false=>"".to_owned(),
                true=>format!(
                    ": of these, {edges_number} connected by edges of {second_graph}",
                    second_graph=other.name,
                    edges_number= match first_merged_components_number {
                        d if d==0=>"none are".to_owned(),
                        d if d==1=>"one is".to_owned(),
                        d if d==first_components_number=>"all components are".to_owned(),
                        _ => format!("{} components are", first_merged_components_number)
                    })
                },
            first_node_percentage=100.0*(overlapping_nodes_number as f64 / self.get_nodes_number() as f64),
            second_node_percentage=100.0*(overlapping_nodes_number as f64 / other.get_nodes_number() as f64),
            first_edge_percentage=100.0*(overlapping_edges_number as f64 / first_edges as f64),
            second_edge_percentage=100.0*(overlapping_edges_number as f64 / second_edges as f64),
        ))
    }

    fn format_list(&self, list: &[String]) -> Result<String, String> {
        if list.is_empty() {
            return Err("Cannot format a list with no elements.".to_owned());
        }
        if list.len() == 1 {
            return Ok(list.first().unwrap().clone());
        }
        let all_minus_last: String = list[0..list.len() - 1].join(", ");
        Ok(format!(
            "{all_minus_last} and {last}",
            all_minus_last = all_minus_last,
            last = list.last().unwrap()
        ))
    }

    /// Return formatted node list.
    ///
    /// # Arguments
    /// * `node_list`: &[NodeT] - list of nodes to be formatted.
    fn format_node_list(&self, node_list: &[NodeT]) -> Result<String, String> {
        self.format_list(
            node_list
                .iter()
                .map(|node_id| {
                    format!(
                        "{node_name} (degree {node_degree})",
                        node_name = self.get_node_name_by_node_id(*node_id).unwrap(),
                        node_degree = self.get_node_degree_by_node_id(*node_id).unwrap()
                    )
                })
                .collect::<Vec<String>>()
                .as_slice(),
        )
    }

    /// Return formatted node type list.
    ///
    /// # Arguments
    /// * `node_types_list`: &[NodeT] - list of nodes to be formatted.
    fn format_node_type_list(
        &self,
        node_types_list: &[(NodeTypeT, usize)],
    ) -> Result<String, String> {
        self.format_list(
            node_types_list
                .iter()
                .map(|(node_type_id, number)| {
                    format!(
                        "{node_type} (nodes number {node_degree})",
                        node_type = self
                            .get_node_type_name_by_node_type_id(*node_type_id)
                            .unwrap(),
                        node_degree = number
                    )
                })
                .collect::<Vec<String>>()
                .as_slice(),
        )
    }

    /// Return formatted edge type list.
    ///
    /// # Arguments
    /// * `edge_types_list`: &[edgeT] - list of edges to be formatted.
    fn format_edge_type_list(
        &self,
        edge_types_list: &[(EdgeTypeT, usize)],
    ) -> Result<String, String> {
        self.format_list(
            edge_types_list
                .iter()
                .map(|(edge_type_id, _)| {
                    self.get_edge_type_name_by_edge_type_id(*edge_type_id)
                        .unwrap()
                })
                .collect::<Vec<String>>()
                .as_slice(),
        )
    }

    /// Return rendered textual report of the graph.
    pub fn textual_report(&self, verbose: bool) -> Result<String, String> {
        {
            let ptr = self.cached_report.read();
            if let Some(report) = &*ptr {
                return Ok(report.clone());
            }
        }

        if !self.has_nodes() {
            return Ok(format!("The graph {} is empty.", self.get_name()));
        }

        let mut ptr = self.cached_report.write();
        // THis is not a duplicate of above because we need to
        // check if another thread already filled the cache
        if let Some(report) = &*ptr {
            return Ok(report.clone());
        }

        let (connected_components_number, minimum_connected_component, maximum_connected_component) =
            self.connected_components_number(verbose);

        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();

        *ptr = Some(format!(
            concat!(
                "The {direction} {graph_type} {name} has {nodes_number} nodes{node_types}{singletons} and {edges_number} {weighted} edges{edge_types}, of which {self_loops}{self_loops_multigraph_connector}{multigraph_edges}. ",
                "The graph is {quantized_density} as it has a density of {density:.5} and {connected_components}. ",
                "The graph median node degree is {median_node_degree}, the mean node degree is {mean_node_degree:.2}, and the node degree mode is {mode_node_degree}. ",
                "The top {most_common_nodes_number} most central nodes are {central_nodes}. ",
                "The hash of the graph is {hash:08x}."
            ),
            hash = hash,
            direction = match self.directed {
                true=> "directed",
                false => "undirected"
            }.to_owned(),
            graph_type = match self.is_multigraph() {
                true=> "multigraph",
                false => "graph"
            }.to_owned(),
            name = self.name,
            nodes_number = self.get_nodes_number(),
            edges_number = self.get_edges_number(),
            weighted = match self.has_weights(){
                true=> "weighted",
                false=> "unweighted"
            }.to_owned(),
            self_loops = match self.has_selfloops() {
                true => format!("{} are self-loops", self.get_self_loop_number()),
                false => "none are self-loops".to_owned()
            },
            self_loops_multigraph_connector = match self.is_multigraph() {
                true => " and ".to_owned(),
                false => "".to_owned()
            },
            multigraph_edges = match self.is_multigraph() {
                true=>match self.get_multigraph_edges_number()>0 {
                    true => format!("{} are parallel", self.get_multigraph_edges_number()),
                    false => "none are parallel".to_owned()
                },
                false=>"".to_owned()
            },
            node_types= match self.get_node_types_number() {
                ntn if ntn==1 => format!(
                    " with a single node type: {node_type}",
                    node_type={
                        let node_types = self.get_node_type_counts()?;
                        self.format_node_type_list(node_types.most_common().as_slice())?
                    }
                ),
                ntn if ntn > 1 => format!(
                    " with {node_types_number} different {multilabel}node types: {most_common_node_types}{unknown_node_types}.",
                    node_types_number=ntn,
                    multilabel=match self.has_multilabel_node_types(){
                        true=>"multi-label ",
                        false=>""
                    },
                    most_common_node_types={
                        let node_types = self.get_node_type_counts()?;
                        let most_common = node_types.most_common();
                        match most_common.len()>5 {
                            true=>format!(" the 5 most common are {}", self.format_node_type_list(most_common[0..5].as_ref())?),
                            false=>self.format_node_type_list(most_common.as_slice())?
                        }
                    },
                    unknown_node_types={
                        match self.has_unknown_node_types(){
                            true=>{
                                let unknown_nodes_number=self.get_unknown_node_types_number();
                                let percentage = 100.0*(unknown_nodes_number as f64 / self.get_nodes_number() as f64);
                                format!(" and there are {} unknown node types ({:.2}%)", unknown_nodes_number, percentage)
                            },
                            false=>"".to_owned()
                        }
                    }
                ),
                _ => "".to_owned()
            },
            singletons = match self.has_singletons() {
                true => format!(
                    " There are {singleton_number} singleton nodes{self_loop_singleton},", 
                    singleton_number=self.get_singleton_nodes_number(),
                    self_loop_singleton=match self.has_singleton_nodes_with_self_loops(){
                        true=>format!(" ({} have self-loops)", match self.get_singleton_nodes_number()==self.get_singleton_nodes_with_self_loops_number(){
                            true=>"all".to_owned(),
                            false=>format!("{} of these", self.get_singleton_nodes_with_self_loops_number())
                        }),
                        false=>"".to_owned()
                    }
                ),
                false => "".to_owned()
            },
            edge_types= match self.get_edge_types_number() {
                etn if etn==1 => format!(
                    " with a single edge type: {edge_type}",
                    edge_type={
                        let edge_types = self.get_edge_type_counts()?;
                        self.format_edge_type_list(edge_types.most_common().as_slice())?
                    }
                ),
                etn if etn > 1 => format!(
                    " with {edge_types_number} different edge types: {most_common_edge_types}{unknown_edge_types}",
                    edge_types_number=etn,
                    most_common_edge_types={
                        let edge_types = self.get_edge_type_counts()?;
                        let most_common = edge_types.most_common();
                        match most_common.len()>5 {
                            true=>format!(" the 5 most common are {}", self.format_edge_type_list(most_common[0..5].as_ref())?),
                            false=>self.format_edge_type_list(most_common.as_slice())?
                        }
                    },
                    unknown_edge_types={
                        match self.has_unknown_edge_types(){
                            true=>{
                                let unknown_edges_number=self.get_unknown_edge_types_number();
                                let percentage = 100.0*(unknown_edges_number as f64 / self.get_directed_edges_number() as f64);
                                format!(". There are {} unknown edge types ({:.2}%).", unknown_edges_number, percentage)
                            },
                            false=>"".to_owned()
                        }
                    }
                ),
                _ => "".to_owned()
            },
            quantized_density = match self.get_density().unwrap() {
                d if d < 0.0001 => "extremely sparse".to_owned(),
                d if d < 0.001 => "quite sparse".to_owned(),
                d if d < 0.01 => "sparse".to_owned(),
                d if d < 0.1 => "dense".to_owned(),
                d if d < 0.5 => "quite dense".to_owned(),
                d if (d - 1.0).abs() < f64::EPSILON => "complete".to_owned(),
                d if d <= 1.0 => "extremely dense".to_owned(),
                d => unreachable!(format!("Unreacheable density case {}", d))
            },
            density=self.get_density().unwrap(),
            connected_components=match connected_components_number> 1{
                true=>format!(
                    "has {components_number} connected components, where the component with most nodes has {maximum_connected_component} and the component with the least nodes has {minimum_connected_component}",
                    components_number=connected_components_number,
                    maximum_connected_component=match maximum_connected_component==1{
                        true=>"a single node".to_owned(),
                        false=>format!("{} nodes", maximum_connected_component)
                    },
                    minimum_connected_component=match minimum_connected_component==1{
                        true=>"a single node".to_owned(),
                        false=>format!("{} nodes", minimum_connected_component)
                    }
                ),
                false=>"is connected, as it has a single component".to_owned()
            },
            median_node_degree=self.get_node_degrees_median().unwrap(),
            mean_node_degree=self.get_node_degrees_mean().unwrap(),
            mode_node_degree=self.get_node_degrees_mode().unwrap(),
            most_common_nodes_number=std::cmp::min(5, self.get_nodes_number()),
            central_nodes = self.format_node_list(self.get_top_k_central_nodes_ids(std::cmp::min(5, self.get_nodes_number())).as_slice())?
        ));

        Ok(ptr.clone().unwrap())
    }
}

use super::*;
use counter::Counter;
use indicatif::ProgressIterator;
use roaring::RoaringBitmap;
use std::collections::HashSet;

/// # remove.
impl Graph {
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// ## Implementation details
    ///
    /// ### How the collapse of multigraphs is handled
    /// We keep only the first edge when a multigraph is collapsed while removing
    /// the edge types, in the order provided when first reading from the CSV file.
    ///
    /// ### Generation of new singleton nodes when removeping edges
    /// Some of the remove operations allowed in this method might lead to the
    /// generation of new singleton nodes that will not be handled within this
    /// function call even if you provide the flag singletons to true, but you
    /// will need to call the method again if you want to get reed of also those
    /// newly created singleton nodes.
    ///
    /// # Arguments
    /// * `allow_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to keep.
    /// * `deny_nodes_set`: Option<HashSet<String>> - Optional set of nodes names to remove.
    /// * `allow_node_types_set`: Option<HashSet<String>> - Optional set of node type names to keep.
    /// * `deny_node_types_set`: Option<HashSet<String>> - Optional set of node type names to remove.
    /// * `allow_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to keep.
    /// * `deny_edge_set`: Option<HashSet<EdgeT>>- Optional set of numeric edge IDs to remove.
    /// * `allow_edge_types_set`: Option<HashSet<String>> - Optional set of edge type names to keep.
    /// * `deny_edge_types_set`: Option<HashSet<String>> - Optional set of edge type names to remove.
    /// * `weights`: bool - whether to remove the weights.
    /// * `node_types`: bool - whether to remove the node types.
    /// * `edge_types`: bool - whether to remove the edge types.
    /// * `singletons`: bool - whether to remove the singleton nodes.
    /// * `selfloops`: bool - whether to remove edges with self-loops.
    /// * `verbose`: bool - whether to show a loading bar while building the graph.
    ///
    pub fn remove(
        &self,
        allow_nodes_set: Option<HashSet<String>>,
        deny_nodes_set: Option<HashSet<String>>,
        allow_node_types_set: Option<HashSet<String>>,
        deny_node_types_set: Option<HashSet<String>>,
        allow_edge_set: Option<HashSet<EdgeT>>,
        deny_edge_set: Option<HashSet<EdgeT>>,
        allow_edge_types_set: Option<HashSet<String>>,
        deny_edge_types_set: Option<HashSet<String>>,
        weights: bool,
        node_types: bool,
        edge_types: bool,
        singletons: bool,
        selfloops: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        let pb_edges = get_loading_bar(
            verbose,
            format!(
                "Building edges of graph {} without required attributes",
                self.name
            )
            .as_ref(),
            self.get_directed_edges_number() as usize,
        );
        let pb_nodes = get_loading_bar(
            verbose,
            format!(
                "Building nodes of graph {} without required attributes",
                self.name
            )
            .as_ref(),
            self.get_nodes_number() as usize,
        );

        Graph::from_string_sorted(
            self.iter_edge_with_type_and_weight(true)
                .progress_with(pb_edges)
                .filter_map(
                    |(edge_id, _, src_name, _, dst_name, _, edge_type, weight)| {
                        // If an allow edge set was provided
                        if let Some(aes) = &allow_edge_set {
                            // We check that the current edge ID is within the edge set.
                            if !aes.contains(&edge_id) {
                                return None;
                            }
                        }
                        // If selfloops need to be filtered out.
                        if selfloops && src_name == dst_name {
                            return None;
                        }
                        // If a deny edge set was provided
                        if let Some(des) = &deny_edge_set {
                            // We check that the current edge ID is NOT within the edge set.
                            if des.contains(&edge_id) {
                                return None;
                            }
                        }
                        // If an allow nodes set was provided
                        if let Some(ans) = &allow_nodes_set {
                            // We check that the current source or destination node name is within the edge set.
                            if !ans.contains(&src_name) || !ans.contains(&dst_name) {
                                return None;
                            }
                        }
                        // If a deny nodes set was provided
                        if let Some(dns) = &deny_nodes_set {
                            // We check that the current source or destination node name is NOT within the edge set.
                            if dns.contains(&src_name) || dns.contains(&dst_name) {
                                return None;
                            }
                        }
                        // If the allow edge types set was provided
                        if let (Some(aets), Some(et)) = (&allow_edge_types_set, &edge_type) {
                            // We check that the current edge type name is within the edge type set.
                            if !aets.contains(et) {
                                return None;
                            }
                        }
                        // If the deny edge types set was provided
                        if let (Some(dets), Some(et)) = (&deny_edge_types_set, &edge_type) {
                            // We check that the current edge type name is NOT within the edge type set.
                            if dets.contains(et) {
                                return None;
                            }
                        }

                        if allow_node_types_set.is_some() || deny_node_types_set.is_some() {
                            let src_node_type = self.get_unchecked_node_type_id_by_node_id(
                                self.get_unchecked_node_id_by_node_name(&src_name),
                            );
                            let dst_node_type = self.get_unchecked_node_type_id_by_node_id(
                                self.get_unchecked_node_id_by_node_name(&dst_name),
                            );
                            // If the graph has node types
                            if let (Some(src_nt), Some(dst_nt)) = (src_node_type, dst_node_type) {
                                let node_type_names = self
                                    .get_node_type_names_by_node_type_ids(
                                        src_nt.into_iter().chain(dst_nt.into_iter()).collect(),
                                    )
                                    .unwrap();
                                // If the allow node types set was provided
                                if let Some(ants) = &allow_node_types_set {
                                    // We check that the current node type name is NOT within the node type set.
                                    if node_type_names
                                        .iter()
                                        .any(|node_type_name| !ants.contains(node_type_name))
                                    {
                                        return None;
                                    }
                                }
                                // If the deny node types set was provided
                                if let Some(dnts) = &deny_node_types_set {
                                    // We check that the current node type name is NOT within the node type set.
                                    if node_type_names
                                        .iter()
                                        .any(|node_type_name| dnts.contains(node_type_name))
                                    {
                                        return None;
                                    }
                                }
                            }
                        }

                        Some(Ok((
                            src_name,
                            dst_name,
                            match edge_types {
                                false => edge_type,
                                true => None,
                            },
                            match weights {
                                false => weight,
                                true => None,
                            },
                        )))
                    },
                ),
            Some(self.iter_nodes().progress_with(pb_nodes).filter_map(
                |(node_id, node_name, _, node_type_names)| {
                    if singletons && self.is_singleton_by_node_name(&node_name).unwrap() {
                        return None;
                    }
                    // If singletons and selfloops need to be removed.
                    // We need to check all the destinations of the node if they are equal
                    // with the source node, as in multigraphs there may be multiple selfloops of different
                    // node types.
                    if singletons
                        && selfloops
                        && self.is_singleton_with_self_loops_by_node_id(node_id)
                    {
                        return None;
                    }
                    if let Some(ans) = &allow_nodes_set {
                        if !ans.contains(&node_name) {
                            return None;
                        }
                    }
                    if let Some(dns) = &deny_nodes_set {
                        if dns.contains(&node_name) {
                            return None;
                        }
                    }
                    if let (Some(ants), Some(nts)) = (&allow_node_types_set, &node_type_names) {
                        // We check that the current node type name is NOT within the node type set.
                        if nts
                            .iter()
                            .any(|node_type_name| !ants.contains(node_type_name))
                        {
                            return None;
                        }
                    }
                    if let (Some(dnts), Some(nts)) = (&deny_node_types_set, &node_type_names) {
                        // We check that the current node type name is NOT within the node type set.
                        if nts
                            .iter()
                            .any(|node_type_name| dnts.contains(node_type_name))
                        {
                            return None;
                        }
                    }
                    Some(Ok((
                        node_name,
                        match node_types {
                            false => node_type_names,
                            true => None,
                        },
                    )))
                },
            )),
            self.directed,
            true,
            false,
            true,
            true,
            true,
            self.get_directed_edges_number() as usize, // Approximation of expected edges number.
            self.get_nodes_number(),                   // Approximation of expected nodes number.
            false,
            false,
            false,
            false,
            self.has_node_types() && !node_types,
            self.has_edge_types() && !edge_types,
            self.has_weights() && !weights,
            // TODO: This may be made more precise!
            true,
            self.has_selfloops() && !selfloops,
            true,
            self.get_name(),
        )
    }

    /// remove all the components that are not connected to interesting
    /// nodes and edges.
    ///
    /// # Arguments
    /// * `node_names` : Option<Vec<String>> - The name of the nodes of which components to keep.
    /// * `node_types` : Option<Vec<String>> - The types of the nodes of which components to keep.
    /// * `edge_types` : Option<Vec<String>> - The types of the edges of which components to keep.
    /// * `minimum_component_size`: Option<NodeT> - Optional, Minimum size of the components to keep.
    /// * `top_k_components`: Option<NodeT> - Optional, number of components to keep sorted by number of nodes.
    /// * `verbose`: bool - whether to show the loading bar.
    pub fn remove_components(
        &self,
        node_names: Option<Vec<String>>,
        node_types: Option<Vec<Option<String>>>,
        edge_types: Option<Vec<Option<String>>>,
        minimum_component_size: Option<NodeT>,
        top_k_components: Option<NodeT>,
        verbose: bool,
    ) -> Result<Graph, String> {
        let mut keep_components = RoaringBitmap::new();
        let components_vector = self.get_node_components_vector(verbose);

        // Extend the components so the include the given node Ids and node types.
        if let Some(node_ids) = self.get_filter_bitmap(node_names, node_types)? {
            keep_components.extend(
                node_ids
                    .iter()
                    .map(|node_id| components_vector[node_id as usize]),
            );
        }

        // Extend the components to keep those that include the given edge types.
        if let Some(ets) = edge_types {
            let edge_types_ids: HashSet<Option<EdgeTypeT>> = self
                .get_edge_type_ids_by_edge_type_names(ets)?
                .into_iter()
                .collect();

            let pb = get_loading_bar(
                verbose,
                &format!(
                    "Computing which components are to keep for the graph {}",
                    &self.name
                ),
                self.get_directed_edges_number() as usize,
            );

            self.iter_edges_with_type_ids(self.directed)
                .progress_with(pb)
                .for_each(|(_, src, dst, edge_type)| {
                    if edge_types_ids.contains(&edge_type) {
                        keep_components.insert(components_vector[src as usize]);
                        keep_components.insert(components_vector[dst as usize]);
                    }
                });
        }

        // Create the components counter
        let component_counts: Vec<(NodeT, NodeT)> =
            Counter::init(components_vector.clone()).most_common_ordered();

        // Insert the top k biggest components components
        if let Some(tkc) = top_k_components {
            for (i, (component_id, _)) in component_counts.iter().enumerate() {
                if i < tkc as usize {
                    keep_components.insert(*component_id);
                }
            }
        }

        // Remove components smaller than the given amount
        if let Some(mcs) = &minimum_component_size {
            component_counts
                .iter()
                .for_each(|(component, component_size)| {
                    if *component_size < *mcs {
                        keep_components.remove(*component);
                    }
                });
        }

        let pb = get_loading_bar(
            verbose,
            &format!(
                "Building edge list with only required components {}",
                &self.name
            ),
            self.get_directed_edges_number() as usize,
        );
        let pb_nodes = get_loading_bar(
            verbose,
            &format!(
                "Building node list with only required components {}",
                &self.name
            ),
            self.get_nodes_number() as usize,
        );

        let min_component_size = keep_components
            .iter()
            .map(|component_id| component_counts[component_id as usize].1)
            .min();

        Graph::from_string_sorted(
            self.iter_edge_with_type_and_weight(true)
                .progress_with(pb)
                .filter_map(
                    |(_, src, src_name, _, dst_name, _, edge_type_name, weight)| {
                        // we just check src because dst is trivially in the same component as src
                        match keep_components.contains(components_vector[src as usize]) {
                            true => Some(Ok((src_name, dst_name, edge_type_name, weight))),
                            false => None,
                        }
                    },
                ),
            Some(self.iter_nodes().progress_with(pb_nodes).filter_map(
                |(node_id, node_name, _, node_type_names)| {
                    match keep_components.contains(components_vector[node_id as usize]) {
                        true => Some(Ok((node_name, node_type_names))),
                        false => None,
                    }
                },
            )),
            self.directed,
            true,
            false,
            true,
            true,
            true,
            self.get_directed_edges_number() as usize, // Approximation of expected edges number.
            self.get_nodes_number(),                   // Approximation of expected nodes number.
            false,
            false,
            false,
            false,
            self.has_node_types(),
            self.has_edge_types(),
            self.has_weights(),
            min_component_size.as_ref().map_or(true, |mcs| *mcs <= 1),
            self.has_singleton_nodes_with_self_loops()
                && min_component_size.as_ref().map_or(true, |mcs| *mcs <= 1),
            self.has_trap_nodes(),
            self.get_name(),
        )
    }
}

use super::*;

/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
pub struct EdgeFileWriter {
    pub(crate) writer: CSVFileWriter,
    pub(crate) sources_column: String,
    pub(crate) sources_column_number: usize,
    pub(crate) destinations_column: String,
    pub(crate) destinations_column_number: usize,
    pub(crate) edge_types_column: String,
    pub(crate) edge_types_column_number: usize,
    pub(crate) weights_column: String,
    pub(crate) weights_column_number: usize,
    pub(crate) numeric_node_ids: bool,
    pub(crate) directed: Option<bool>,
}

impl EdgeFileWriter {
    /// Return new EdgeFileWriter object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> EdgeFileWriter {
        EdgeFileWriter {
            writer: CSVFileWriter::new(path),
            sources_column: "subject".to_string(),
            sources_column_number: 0,
            destinations_column: "object".to_string(),
            destinations_column_number: 1,
            edge_types_column: "label".to_string(),
            edge_types_column_number: 2,
            weights_column: "weight".to_string(),
            weights_column_number: 3,
            numeric_node_ids: false,
            directed: None,
        }
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column<S: Into<String>>(
        mut self,
        sources_column: Option<S>,
    ) -> EdgeFileWriter {
        if let Some(column) = sources_column {
            self.sources_column = column.into();
        }
        self
    }

    /// Set the column of the source nodes.
    ///
    /// # Arguments
    ///
    /// * sources_column_number: Option<String> - The source nodes column to use for the file.
    ///
    pub fn set_sources_column_number(
        mut self,
        sources_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = sources_column_number {
            self.sources_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column<S: Into<String>>(
        mut self,
        destinations_column: Option<S>,
    ) -> EdgeFileWriter {
        if let Some(column) = destinations_column {
            self.destinations_column = column.into();
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * destinations_column_number: Option<String> - The node types column to use for the file.
    ///
    pub fn set_destinations_column_number(
        mut self,
        destinations_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = destinations_column_number {
            self.destinations_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column<S: Into<String>>(
        mut self,
        edge_type_column: Option<S>,
    ) -> EdgeFileWriter {
        if let Some(column) = edge_type_column {
            self.edge_types_column = column.into();
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * edge_types_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_edge_types_column_number(
        mut self,
        edge_type_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = edge_type_column_number {
            self.edge_types_column_number = column_number;
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_weights_column<S: Into<String>>(
        mut self,
        weights_column: Option<S>,
    ) -> EdgeFileWriter {
        if let Some(column) = weights_column {
            self.weights_column = column.into();
        }
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * weights_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_weights_column_number(
        mut self,
        weights_column_number: Option<usize>,
    ) -> EdgeFileWriter {
        if let Some(column_number) = weights_column_number {
            self.weights_column_number = column_number;
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> EdgeFileWriter {
        if let Some(v) = verbose {
            self.writer.verbose = v;
        }
        self
    }

    /// Set the numeric_id.
    ///
    /// # Arguments
    ///
    /// * numeric_id: Option<bool> - whether to convert numeric Ids to Node Id.
    ///
    pub fn set_numeric_node_ids(mut self, numeric_node_ids: Option<bool>) -> EdgeFileWriter {
        if let Some(nni) = numeric_node_ids {
            self.numeric_node_ids = nni;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(mut self, separator: Option<S>) -> EdgeFileWriter {
        if let Some(v) = separator {
            self.writer.separator = v.into();
        }
        self
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - whether to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> EdgeFileWriter {
        if let Some(v) = header {
            self.writer.header = v;
        }
        self
    }

    /// Set the directed.
    ///
    /// # Arguments
    ///
    /// * directed: Option<bool> - whether to write out the graph as directed or not.
    ///
    pub fn set_directed(mut self, directed: Option<bool>) -> EdgeFileWriter {
        self.directed = directed;
        self
    }

    /// Write edge file.
    ///  
    /// # Arguments
    ///
    /// * `graph`: &Graph - the graph to write out.
    pub fn dump(&self, graph: &Graph) -> Result<(), String> {
        let directed: bool = self.directed.unwrap_or_else(|| graph.is_directed());
        // build the header
        let mut header = vec![
            (self.sources_column.clone(), self.sources_column_number),
            (
                self.destinations_column.clone(),
                self.destinations_column_number,
            ),
        ];

        if graph.has_edge_types() {
            header.push((
                self.edge_types_column.clone(),
                self.edge_types_column_number,
            ));
        }

        if graph.has_weights() {
            header.push((self.weights_column.clone(), self.weights_column_number));
        }

        let number_of_columns = 1 + header.iter().map(|(_, i)| i).max().unwrap();

        self.writer.write_lines(
            graph.get_directed_edges_number() as usize,
            compose_lines(number_of_columns, header),
            graph.iter_edge_with_type_and_weight_ids(directed).map(
                |(_, src, dst, edge_type, weight)| {
                    let mut line = vec![
                        (
                            match self.numeric_node_ids {
                                true => src.to_string(),
                                false => graph.nodes.unchecked_translate(src),
                            },
                            self.sources_column_number,
                        ),
                        (
                            match self.numeric_node_ids {
                                true => dst.to_string(),
                                false => graph.nodes.unchecked_translate(dst),
                            },
                            self.destinations_column_number,
                        ),
                    ];

                    if let Some(ets) = &graph.edge_types {
                        line.push((
                            edge_type.map_or("".to_string(), |et| ets.unchecked_translate(et)),
                            self.edge_types_column_number,
                        ));
                    }

                    if let Some(w) = weight {
                        line.push((w.to_string(), self.weights_column_number));
                    }

                    compose_lines(number_of_columns, line)
                },
            ),
        )
    }
}

use super::*;

impl Graph {
    /// Set the name of the graph.
    ///
    /// # Arguments
    ///
    /// * name: String - Name of the graph.
    pub fn set_name(&mut self, name: String) {
        self.invalidate_report();
        self.name = name;
    }

    /// Invalidate the cache for the textual report.
    /// This should be called as the first line of every methods that either get
    /// a mutable reference to self or get ownership of self.
    pub(crate) fn invalidate_report(&self)  {
        *self.cached_report.write() = None;
    }

    /// Replace all edge types (if present) and set all the edge to edge_type.
    ///
    /// # Arguments
    /// - `edge_type`: String - The edge type to assing to all the edges.
    pub fn set_all_edge_types<S: Into<String>>(mut self, edge_type: S) -> Graph {
        self.invalidate_report();
        let mut vocabulary = Vocabulary::default();
        vocabulary.insert(edge_type.into()).unwrap();
        vocabulary.build_reverse_mapping().unwrap();
        let edge_types = EdgeTypeVocabulary::from_structs(
            vec![Some(0); self.get_directed_edges_number() as usize],
            vocabulary,
        );
        self.edge_types = Some(edge_types);
        self
    }

    /// Replace all node types (if present) and set all the node to node_type.
    ///
    /// # Arguments
    /// - `node_type`: String - The node type to assing to all the nodes.
    pub fn set_all_node_types<S: Into<String>>(mut self, node_type: S) -> Graph {
        self.invalidate_report();
        let mut vocabulary = Vocabulary::default();
        vocabulary.insert(node_type.into()).unwrap();
        vocabulary.build_reverse_mapping().unwrap();
        let node_types = NodeTypeVocabulary::from_structs(
            vec![Some(vec![0]); self.get_nodes_number() as usize],
            Some(vocabulary),
        );
        self.node_types = node_types;
        self
    }
}

use super::*;
use log::info;
use rayon::prelude::*;
use vec_rand::sample_f32 as sample;
use vec_rand::sample_uniform;
use vec_rand::splitmix64;

#[inline(always)]
fn update_return_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    src: NodeT,
    dst: NodeT,
    return_weight: ParamsT,
    has_selfloop: bool,
) {
    if let Ok(mut i) = destinations.binary_search(&src) {
        let mut j = i;
        while j > 0 && destinations[j] == src {
            transition[j] *= return_weight;
            j -= 1;
        }
        i += 1;
        while i < destinations.len() && destinations[i] == src {
            transition[i] *= return_weight;
            i += 1;
        }
    }

    if src != dst && has_selfloop {
        if let Ok(mut i) = destinations.binary_search(&dst) {
            let mut j = i;
            while j > 0 && destinations[j] == dst {
                transition[j] *= return_weight;
                j -= 1;
            }
            i += 1;
            while i < destinations.len() && destinations[i] == dst {
                transition[i] *= return_weight;
                i += 1;
            }
        }
    }
}

#[inline(always)]
fn rust_update_explore_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    explore_weight: ParamsT,
    src: NodeT,
    dst: NodeT,
) {
    let mut i = 0;
    let mut j = 0;
    let mut v1: NodeT;
    let mut v2: NodeT;
    //############################################################
    //# Handling of the Q parameter: the explore coefficient     #
    //############################################################
    // This coefficient increases the probability of switching
    // to nodes not locally seen.
    while i < destinations.len() && j < previous_destinations.len() {
        v1 = destinations[i];
        v2 = previous_destinations[j];
        if v1 <= v2 {
            let is_less = v1 < v2;
            if is_less && v1 != src && v1 != dst {
                transition[i] *= explore_weight;
            }
            j += !is_less as usize;
            i += 1;
        } else {
            j += 1;
        }
    }
    for k in i..destinations.len() {
        v1 = destinations[k];
        transition[k] *= 1.0 + (v1 != src && v1 != dst) as u64 as WeightT * (explore_weight - 1.0);
    }
}

#[inline(always)]
fn rust_update_return_explore_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    return_weight: ParamsT,
    explore_weight: ParamsT,
    src: NodeT,
    dst: NodeT,
) {
    let mut i = 0;
    let mut j = 0;
    let mut v1: NodeT;
    let mut v2: NodeT;
    //############################################################
    //# Handling of the Q parameter: the explore coefficient     #
    //############################################################
    // This coefficient increases the probability of switching
    // to nodes not locally seen.
    while i < destinations.len() && j < previous_destinations.len() {
        v1 = destinations[i];
        v2 = previous_destinations[j];
        if v1 == src || v1 == dst {
            transition[i] *= return_weight;
            i += 1;
            continue;
        }
        if v1 <= v2 {
            let is_less = v1 < v2;
            if is_less {
                transition[i] *= explore_weight;
            }
            j += !is_less as usize;
            i += 1;
        } else {
            j += 1;
        }
    }
    for k in i..destinations.len() {
        v1 = destinations[k];
        if v1 == src || v1 == dst {
            transition[k] *= return_weight;
        } else {
            transition[k] *= explore_weight;
        }
    }
}

extern "C" {
    fn c_update_explore_weight_transition(
        transition: *const f32,
        destinations: *const u32,
        destinations_len: u32,
        previous_destinations: *const u32,
        previous_destinations_len: u32,
        explore_weight: f32,
        src: u32,
        dst: u32,
    );
    fn c_update_return_explore_weight_transition(
        transition: *const f32,
        destinations: *const u32,
        destinations_len: u32,
        previous_destinations: *const u32,
        previous_destinations_len: u32,
        explore_weight: f32,
        return_weight: f32,
        src: u32,
        dst: u32,
    );
}

fn update_explore_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    explore_weight: ParamsT,
    src: NodeT,
    dst: NodeT,
) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                c_update_explore_weight_transition(
                    transition.as_ptr(),
                    destinations.as_ptr(),
                    destinations.len() as u32,
                    previous_destinations.as_ptr(),
                    previous_destinations.len() as u32,
                    explore_weight,
                    src,
                    dst,
                );
            }
            return;
        }
    }
    rust_update_explore_weight_transition(
        transition,
        destinations,
        previous_destinations,
        explore_weight,
        src,
        dst,
    );
}

fn update_return_explore_weight_transition(
    transition: &mut Vec<WeightT>,
    destinations: &[NodeT],
    previous_destinations: &[NodeT],
    return_weight: ParamsT,
    explore_weight: ParamsT,
    src: NodeT,
    dst: NodeT,
) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                c_update_return_explore_weight_transition(
                    transition.as_ptr(),
                    destinations.as_ptr(),
                    destinations.len() as u32,
                    previous_destinations.as_ptr(),
                    previous_destinations.len() as u32,
                    explore_weight,
                    return_weight,
                    src,
                    dst,
                );
            }
            return;
        }
    }
    rust_update_return_explore_weight_transition(
        transition,
        destinations,
        previous_destinations,
        return_weight,
        explore_weight,
        src,
        dst,
    );
}

#[cfg(test)]
mod tests {
    use super::update_explore_weight_transition;
    use super::update_return_explore_weight_transition;
    use super::update_return_weight_transition;
    use super::WeightT;

    #[test]
    fn test_update_explore_weight_transition() {
        let destinations = vec![
            1, 2, 3, 4, 4, 4, 5, 6, 100, 101, 101, 101, 101, 101, 101, 101, 101, 101, 101, 101,
            101, 101, 101, 101,
        ];
        let previous_destinations = vec![2, 4, 4, 4];
        let mut transitions = (0..destinations.len())
            .map(|_| 1.0)
            .collect::<Vec<WeightT>>();
        update_explore_weight_transition(
            &mut transitions,
            &destinations,
            &previous_destinations,
            2.0,
            6,
            100,
        );
        assert_eq!(
            transitions,
            vec![
                2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
                2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0
            ]
        )
    }

    #[test]
    fn test_update_return_explore_weight_transition() {
        let destinations = vec![1, 2, 3, 4, 4, 4, 5, 6, 100];
        let previous_destinations = vec![2, 4, 4, 4];
        let mut transitions = (0..destinations.len())
            .map(|_| 1.0)
            .collect::<Vec<WeightT>>();
        update_return_explore_weight_transition(
            &mut transitions,
            &destinations,
            &previous_destinations,
            3.0,
            2.0,
            6,
            100,
        );
        assert_eq!(
            transitions,
            vec![2.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 3.0, 3.0]
        )
    }

    #[test]
    fn test_update_return_weight_transition() {
        let destinations = vec![1, 2, 3, 4, 4, 4, 5, 6, 100];
        let mut transitions = (0..destinations.len())
            .map(|_| 1.0)
            .collect::<Vec<WeightT>>();
        update_return_weight_transition(&mut transitions, &destinations, 6, 2, 2.0, true);
        assert_eq!(
            transitions,
            vec![1.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 1.0]
        )
    }
}

impl Graph {
    /// Return the base weighted transitions.
    ///
    /// # Arguments
    ///
    /// * min_edge_id: EdgeT - The minimum edge id.
    /// * max_edge_id: EdgeT - The maximum edge id.
    ///
    fn get_weighted_transitions(
        &self,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        probabilistic_indices: &Option<Vec<u64>>,
    ) -> Vec<WeightT> {
        match &probabilistic_indices {
            Some(indices) => match &self.weights {
                Some(ws) => indices
                    .iter()
                    .map(|edge_id| ws[*edge_id as usize])
                    .collect(),
                // Otherwise we return an uniform vector.
                None => vec![1.0; indices.len()],
            },
            None => match &self.weights {
                Some(ws) => ws[(min_edge_id as usize)..(max_edge_id as usize)].to_vec(),
                // Otherwise we return an uniform vector.
                None => vec![1.0; (max_edge_id - min_edge_id) as usize],
            },
        }
    }

    /// Updates the the transitions probability score for the change of the node type.
    ///
    /// Specifically, we multiply the transition score by the given `change_node_type_weight`
    /// when the node type changes.
    ///
    /// # Arguments
    ///
    /// node: NodeT - Source node.
    /// transition: &mut Vec<WeightT> - Vector of transitions to update.
    /// destinations: impl Iterator<Item = NodeT> - Iterator of the destinations.
    /// change_node_type_weight: ParamsT - The weight to multiply the transition by if there is a change of node type.
    ///
    fn update_node_transition(
        &self,
        node: NodeT,
        transition: &mut Vec<WeightT>,
        destinations: impl Iterator<Item = NodeT>,
        change_node_type_weight: ParamsT,
    ) {
        //############################################################
        //# Handling of the change node type parameter               #
        //############################################################

        if not_one(change_node_type_weight) {
            // If the node types were given:
            if let Some(nt) = &self.node_types {
                // if the destination node type matches the neighbour
                // destination node type (we are not changing the node type)
                // we weigth using the provided change_node_type_weight weight.

                transition
                    .iter_mut()
                    .zip(destinations)
                    .for_each(|(transition_value, dst)| {
                        if nt.ids[node as usize] != nt.ids[dst as usize] {
                            *transition_value *= change_node_type_weight
                        }
                    });
            }
        }
    }

    /// Return the node transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions, if this is bigger that the number of nodes it will panic.
    /// * walk_weights: WalkWeights, the weights for the weighted random walks.
    ///
    fn get_node_transition(
        &self,
        node: NodeT,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        probabilistic_indices: &Option<Vec<u64>>,
    ) -> Vec<WeightT> {
        // Retrieve the data to compute the update transition
        let mut transition =
            self.get_weighted_transitions(min_edge_id, max_edge_id, probabilistic_indices);

        // Compute the transition weights relative to the node weights.
        self.update_node_transition(
            node,
            &mut transition,
            destinations.iter().cloned(),
            walk_weights.change_node_type_weight,
        );

        transition
    }

    /// Return the edge transition weights and the related node and edges.
    ///
    /// # Arguments
    ///
    /// * edge: EdgeT - the previous edge from which to compute the transitions.
    /// * weights: WalkWeights - Weights to use for the weighted walk.
    fn get_edge_transition(
        &self,
        src: NodeT,
        dst: NodeT,
        edge_id: EdgeT,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        previous_destinations: &[NodeT],
        probabilistic_indices: &Option<Vec<u64>>,
        has_selfloop: bool,
    ) -> (Vec<WeightT>, EdgeT) {
        let mut transition =
            self.get_weighted_transitions(min_edge_id, max_edge_id, probabilistic_indices);

        // Compute the transition weights relative to the node weights.
        self.update_node_transition(
            dst,
            &mut transition,
            destinations.iter().cloned(),
            walk_weights.change_node_type_weight,
        );

        //############################################################
        //# Handling of the change edge type parameter               #
        //############################################################

        // If the edge types were given:
        if not_one(walk_weights.change_edge_type_weight) {
            if let Some(ets) = &self.edge_types {
                //# If the neighbour edge type matches the previous
                //# edge type (we are not changing the edge type)
                //# we weigth using the provided change_edge_type_weight weight.
                let this_type: Option<EdgeTypeT> = ets.ids[edge_id as usize];
                transition
                    .iter_mut()
                    .zip(min_edge_id..max_edge_id)
                    .for_each(|(transition_value, edge_id)| {
                        if this_type == ets.ids[edge_id as usize] {
                            *transition_value /= walk_weights.change_edge_type_weight
                        }
                    });
            }
        }

        //###############################################################
        //# Handling of the P & Q parameters: the node2vec coefficients #
        //###############################################################
        match (
            not_one(walk_weights.return_weight),
            not_one(walk_weights.explore_weight),
        ) {
            (false, false) => {}
            (false, true) => {
                update_explore_weight_transition(
                    &mut transition,
                    destinations,
                    previous_destinations,
                    walk_weights.explore_weight,
                    src,
                    dst,
                );
            }
            (true, false) => {
                update_return_weight_transition(
                    &mut transition,
                    destinations,
                    src,
                    dst,
                    walk_weights.return_weight,
                    has_selfloop,
                );
            }
            (true, true) => {
                update_return_explore_weight_transition(
                    &mut transition,
                    destinations,
                    previous_destinations,
                    walk_weights.return_weight,
                    walk_weights.explore_weight,
                    src,
                    dst,
                );
            }
        }

        (transition, min_edge_id)
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * random_state: u64, the random_state to use for extracting the node.
    ///
    fn extract_uniform_node(&self, node: NodeT, random_state: u64) -> NodeT {
        let (min_edge, max_edge) = self.get_minmax_edge_ids_by_source_node_id(node);
        let sampled_offset = sample_uniform((max_edge - min_edge) as u64, random_state);

        match self
            .cached_destinations
            .as_ref()
            .and_then(|cds| cds.get(&node))
        {
            Some(dsts) => dsts[sampled_offset],
            None => self
                .get_unchecked_destination_node_id_by_edge_id(min_edge + sampled_offset as EdgeT),
        }
    }

    /// Return new sampled node with the transition edge used.
    ///
    /// # Arguments
    ///
    /// * node: NodeT, the previous node from which to compute the transitions.
    /// * random_state: usize, the random_state to use for extracting the node.
    /// * walk_weights: WalkWeights, the weights for the weighted random walks.
    fn extract_node(
        &self,
        node: NodeT,
        random_state: u64,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        probabilistic_indices: &Option<Vec<u64>>,
    ) -> (NodeT, EdgeT) {
        let mut weights = self.get_node_transition(
            node,
            walk_weights,
            min_edge_id,
            max_edge_id,
            destinations,
            probabilistic_indices,
        );
        let sampled_offset = sample(&mut weights, random_state);
        let edge_id = match probabilistic_indices {
            Some(inds) => inds[sampled_offset],
            None => min_edge_id + sampled_offset as EdgeT,
        };

        let destination = match self
            .cached_destinations
            .as_ref()
            .and_then(|cds| cds.get(&node))
        {
            Some(dsts) => dsts[sampled_offset],
            None => self.get_unchecked_destination_node_id_by_edge_id(edge_id),
        };
        (destination, edge_id)
    }

    /// Return new random edge with given weights.
    ///
    /// # Arguments
    ///
    /// * `src`: NodeT - Current source node id.
    /// * `dst`: NodeT - Current destination node id.
    /// * `edge`: EdgeT - Current edge id.
    /// * `random_state`: NodeT - The random state to use to sample the next edge id.
    /// * `walk_weights`: &WalkWeights - Struct with the weights to use to update the transitions.
    /// * `min_edge_id`: EdgeT - Minimum edge id to sample for given destination node id.
    /// * `max_edge_id`: EdgeT - Maximum edge id to sample for given destination node id.
    /// * `destinations`: &[NodeT] - Current destinations slice.
    /// * `previous_destinations`: &[NodeT] - Previous destination slice.
    /// * `probabilistic_indices`: &Option<Vec<u64>> - Probabilistic indices, used when max neighbours is provided.
    ///
    fn extract_edge(
        &self,
        src: NodeT,
        dst: NodeT,
        edge: EdgeT,
        random_state: u64,
        walk_weights: &WalkWeights,
        min_edge_id: EdgeT,
        max_edge_id: EdgeT,
        destinations: &[NodeT],
        previous_destinations: &[NodeT],
        probabilistic_indices: &Option<Vec<u64>>,
    ) -> (NodeT, EdgeT) {
        let (mut weights, min_edge_id) = self.get_edge_transition(
            src,
            dst,
            edge,
            walk_weights,
            min_edge_id,
            max_edge_id,
            destinations,
            previous_destinations,
            probabilistic_indices,
            self.has_selfloops(),
        );
        let sampled_offset = sample(&mut weights, random_state as u64);
        let edge_id = match probabilistic_indices {
            Some(inds) => inds[sampled_offset],
            None => min_edge_id + sampled_offset as EdgeT,
        };
        let destination = match self
            .cached_destinations
            .as_ref()
            .and_then(|cds| cds.get(&dst))
        {
            Some(dsts) => dsts[sampled_offset],
            None => self.get_unchecked_destination_node_id_by_edge_id(edge_id),
        };
        (destination, edge_id)
    }

    /// Return vector of walks run on each non-trap node of the graph.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    pub fn random_walks_iter<'a>(
        &'a self,
        quantity: NodeT,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a, String> {
        if !self.has_edges() {
            return Err(
                "It does not make sense to compute a random walk on an empty graph.".to_string(),
            );
        }
        let factor = 0xDEAD;
        let random_state = splitmix64(parameters.random_state.wrapping_mul(factor) as u64);
        self.walk_iter(
            quantity,
            move |index| {
                let local_index = index % quantity;
                let random_source_id =
                    splitmix64(random_state + local_index.wrapping_mul(factor) as u64) as NodeT;
                (
                    splitmix64(random_state + index.wrapping_mul(factor) as u64),
                    self.get_unique_source(
                        random_source_id % self.get_unique_source_nodes_number(),
                    ),
                )
            },
            parameters,
        )
    }

    /// Return vector of walks run on a random subset of the not trap nodes.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    pub fn complete_walks_iter<'a>(
        &'a self,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a, String> {
        if !self.has_edges() {
            return Err(
                "It does not make sense to compute a random walk on an empty graph.".to_string(),
            );
        }
        let factor = 0xDEAD;
        let random_state = splitmix64(parameters.random_state.wrapping_mul(factor) as u64);
        self.walk_iter(
            self.get_unique_source_nodes_number(),
            move |index| {
                (
                    splitmix64(random_state + index.wrapping_mul(factor) as u64),
                    self.get_unique_source(index as NodeT % self.get_unique_source_nodes_number()),
                )
            },
            parameters,
        )
    }

    /// Returns vector of walks.
    ///
    /// # Arguments
    ///
    /// * parameters: WalksParameters - the weighted walks parameters.
    ///
    fn walk_iter<'a>(
        &'a self,
        quantity: NodeT,
        to_node: impl Fn(NodeT) -> (u64, NodeT) + Sync + Send + 'a,
        parameters: &'a WalksParameters,
    ) -> Result<impl IndexedParallelIterator<Item = Vec<NodeT>> + 'a, String> {
        if self.directed {
            return Err("Not supporting directed walks as of now.".to_owned());
        }

        // Validate if given parameters are compatible with current graph.
        parameters.validate(&self)?;

        let total_iterations = quantity * parameters.iterations;
        info!("Starting random walk.");

        // If the graph does not have any weights and the parameters
        // for the walks are all equal to 1, we can use the first-order
        // random walk algorithm.
        let use_uniform = !self.has_weights() && parameters.is_first_order_walk();

        let walks = (0..total_iterations).into_par_iter().map(move |index| {
            let (random_state, node) = to_node(index);
            let mut walk = match use_uniform {
                true => self.uniform_walk(
                    node,
                    random_state,
                    parameters.single_walk_parameters.walk_length,
                ),
                false => self.single_walk(node, random_state, &parameters.single_walk_parameters),
            };

            if let Some(dense_node_mapping) = &parameters.dense_node_mapping {
                walk.iter_mut()
                    .for_each(|node| *node = *dense_node_mapping.get(node).unwrap());
            }
            walk
        });

        Ok(walks)
    }

    /// Returns single walk from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * random_state: usize, the random_state to use for extracting the nodes and edges.
    /// * parameters: SingleWalkParameters - Parameters for the single walk.
    ///
    fn single_walk(
        &self,
        node: NodeT,
        random_state: u64,
        parameters: &SingleWalkParameters,
    ) -> Vec<NodeT> {
        let (min_edge_id, max_edge_id, destinations, indices) =
            self.get_node_edges_and_destinations(parameters.max_neighbours, random_state, node);
        let (dst, edge) = self.extract_node(
            node,
            random_state,
            &parameters.weights,
            min_edge_id,
            max_edge_id,
            self.get_destinations_slice(min_edge_id, max_edge_id, node, &destinations),
            &indices,
        );

        let mut result = Vec::with_capacity(parameters.walk_length as usize);
        result.push(node);
        result.push(dst);
        // We iterate two times before because we need to parse the two initial nodes

        let mut previous_min_edge_id = min_edge_id;
        let mut previous_max_edge_id = max_edge_id;
        let mut previous_destinations = destinations;
        let mut previous_src = node;
        let mut previous_dst = dst;
        let mut previous_edge = edge;

        for i in 2..parameters.walk_length {
            let (min_edge_id, max_edge_id, destinations, indices) = self
                .get_node_edges_and_destinations(
                    parameters.max_neighbours,
                    random_state + i,
                    previous_dst,
                );
            let (dst, edge) = self.extract_edge(
                previous_src,
                previous_dst,
                previous_edge,
                random_state + i,
                &parameters.weights,
                min_edge_id,
                max_edge_id,
                self.get_destinations_slice(min_edge_id, max_edge_id, previous_dst, &destinations),
                self.get_destinations_slice(
                    previous_min_edge_id,
                    previous_max_edge_id,
                    previous_src,
                    &previous_destinations,
                ),
                &indices,
            );

            previous_min_edge_id = min_edge_id;
            previous_max_edge_id = max_edge_id;
            previous_destinations = destinations;
            previous_src = previous_dst;
            previous_dst = dst;
            previous_edge = edge;
            result.push(dst);
        }

        result
    }

    /// Returns single walk from given node.
    ///
    /// This method assumes that there are no traps in the graph.
    ///
    /// # Arguments
    ///
    /// * node: NodeT - Node from where to start the random walks.
    /// * random_state: usize - the random_state to use for extracting the nodes and edges.
    /// * walk_length: u64 - Length of the random walk.
    ///
    fn uniform_walk(&self, node: NodeT, random_state: u64, walk_length: u64) -> Vec<NodeT> {
        // We iterate one time before because we need to parse the initial node.
        (0..1)
            .map(move |_| node)
            .chain((1..walk_length).scan(node, move |node, iteration| {
                *node = self.extract_uniform_node(*node, random_state + iteration);
                Some(*node)
            }))
            .collect()
    }
}

use super::*;

#[inline(always)]
pub(crate) fn encode_edge(src: NodeT, dst: NodeT, node_bits: u8) -> EdgeT {
    ((src as EdgeT) << node_bits) | dst as EdgeT
}

#[inline(always)]
pub(crate) fn encode_max_edge(node: NodeT, node_bits: u8) -> EdgeT {
    ((node as EdgeT) << node_bits) | node as EdgeT
}

#[inline(always)]
pub(crate) fn decode_edge(edge: u64, node_bits: u8, node_bit_mask: u64) -> (NodeT, NodeT) {
    (
        (edge >> node_bits) as NodeT,
        (edge & node_bit_mask) as NodeT,
    )
}

#[inline(always)]
pub(crate) fn get_node_bits(top_node: NodeT) -> u8 {
    (1.0 + top_node as f64).log2().ceil() as u8
}

impl Graph {
    #[inline(always)]
    pub fn encode_edge(&self, src: NodeT, dst: NodeT) -> u64 {
        encode_edge(src, dst, self.node_bits)
    }

    #[inline(always)]
    pub fn decode_edge(&self, edge: u64) -> (NodeT, NodeT) {
        decode_edge(edge, self.node_bits, self.node_bit_mask)
    }

    #[inline(always)]
    pub(crate) fn get_node_ids_from_edge_id(&self, edge_id: EdgeT) -> (NodeT, NodeT) {
        if let (Some(sources), Some(destinations)) = (&self.sources, &self.destinations) {
            return (sources[edge_id as usize], destinations[edge_id as usize]);
        }
        self.decode_edge(self.edges.unchecked_select(edge_id))
    }

    #[inline(always)]
    pub fn get_edge_id_by_node_ids(&self, src: NodeT, dst: NodeT) -> Result<EdgeT, String> {
        match self
            .edges
            .rank(self.encode_edge(src, dst))
            .map(|value| value as EdgeT) {
                Some(edge_id) => Ok(edge_id),
                None => Err(format!("The edge composed by the source node {} and destination node {} does not exist in this graph.", src, dst))
            }
    }

    #[inline(always)]
    pub(crate) fn get_unchecked_edge_id_from_tuple(&self, src: NodeT, dst: NodeT) -> EdgeT {
        self.edges.unchecked_rank(self.encode_edge(src, dst)) as EdgeT
    }

    #[inline(always)]
    pub(crate) fn get_unique_source(&self, source_id: NodeT) -> NodeT {
        self.unique_sources
            .as_ref()
            .map_or(source_id, |x| x.unchecked_select(source_id as u64) as NodeT)
    }
}

use super::*;
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;

impl Graph {
    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// # Arguments
    /// `removed_existing_edges`: Option<bool> - whether to filter out the existing edges. By default, true.
    /// `first_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the first set of nodes of the graph.
    /// `second_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the second set of nodes of the graph.
    /// `first_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the first set of nodes of the graph.
    /// `second_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the second set of nodes of the graph.
    pub fn get_bipartite_edges(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> Result<Vec<Vec<NodeT>>, String> {
        let removed_existing_edges_unwrapped = removed_existing_edges.unwrap_or(true);
        let (first_nodes, second_nodes): (Vec<NodeT>, Vec<NodeT>) = [
            (first_nodes_set, first_node_types_set),
            (second_nodes_set, second_node_types_set),
        ]
        .iter()
        .map(|(node_set, node_type_set)| {
            self.iter_nodes()
                .filter_map(|(node_id, node_name, _, node_type)| {
                    if let Some(ans) = &node_set {
                        if !ans.contains(&node_name) {
                            return None;
                        }
                    }
                    if let (Some(ants), Some(nt)) = (&node_type_set, &node_type) {
                        if nt
                            .iter()
                            .any(|node_type_name| !ants.contains(node_type_name))
                        {
                            return None;
                        }
                    }
                    Some(node_id)
                })
                .collect::<Vec<NodeT>>()
        })
        .collect_tuple()
        .unwrap();

        if first_nodes.is_empty() {
            return Err("The first nodes set of required bipartite graph is empty!".to_owned());
        }

        if second_nodes.is_empty() {
            return Err("The second nodes set of required bipartite graph is empty!".to_owned());
        }

        if first_nodes
            .par_iter()
            .any(|src| second_nodes.binary_search(src).is_ok())
        {
            return Err(
                "The giving node sets of the required bipartite graph have shared nodes."
                    .to_owned(),
            );
        }

        Ok(first_nodes
            .par_iter()
            .flat_map(|src| {
                second_nodes
                    .iter()
                    .filter_map(|dst| {
                        if removed_existing_edges_unwrapped && self.has_edge_by_node_ids(*src, *dst)
                        {
                            return None;
                        }
                        Some(vec![*src, *dst])
                    })
                    .collect::<Vec<Vec<NodeT>>>()
            })
            .collect())
    }

    /// Return vector of tuple of Node IDs that form the edges of the required bipartite graph.
    ///
    /// # Arguments
    /// `removed_existing_edges`: Option<bool> - whether to filter out the existing edges. By default, true.
    /// `first_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the first set of nodes of the graph.
    /// `second_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the second set of nodes of the graph.
    /// `first_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the first set of nodes of the graph.
    /// `second_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the second set of nodes of the graph.
    pub fn get_bipartite_edge_names(
        &self,
        removed_existing_edges: Option<bool>,
        first_nodes_set: Option<HashSet<String>>,
        second_nodes_set: Option<HashSet<String>>,
        first_node_types_set: Option<HashSet<String>>,
        second_node_types_set: Option<HashSet<String>>,
    ) -> Result<Vec<Vec<String>>, String> {
        Ok(self
            .get_bipartite_edges(
                removed_existing_edges,
                first_nodes_set,
                second_nodes_set,
                first_node_types_set,
                second_node_types_set,
            )?
            .iter()
            .map(|nodes| {
                nodes
                    .iter()
                    .map(|node| self.get_node_name_by_node_id(*node).unwrap())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>())
    }

    /// Return vector of tuple of Node IDs that form the edges of the required star.
    ///
    /// # Arguments
    /// `central_node`: String - Name of the node to use as center of the star.
    /// `removed_existing_edges`: Option<bool> - whether to filter out the existing edges. By default, true.
    /// `star_points_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the set of star points.
    /// `star_points_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the set of star points.
    pub fn get_star_edges(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> Result<Vec<Vec<NodeT>>, String> {
        self.get_bipartite_edges(
            removed_existing_edges,
            Some(vec![central_node].into_iter().collect::<HashSet<String>>()),
            star_points_nodes_set,
            None,
            star_points_node_types_set,
        )
    }

    /// Return vector of tuple of Node names that form the edges of the required star.
    ///
    /// # Arguments
    /// `central_node`: String - Name of the node to use as center of the star.
    /// `removed_existing_edges`: Option<bool> - whether to filter out the existing edges. By default, true.
    /// `star_points_nodes_set`: Option<HashMap<String>> - Optional set of nodes to use to create the set of star points.
    /// `star_points_node_types_set`: Option<HashMap<String>> - Optional set of node types to create the set of star points.
    pub fn get_star_edge_names(
        &self,
        central_node: String,
        removed_existing_edges: Option<bool>,
        star_points_nodes_set: Option<HashSet<String>>,
        star_points_node_types_set: Option<HashSet<String>>,
    ) -> Result<Vec<Vec<String>>, String> {
        self.get_bipartite_edge_names(
            removed_existing_edges,
            Some(vec![central_node].into_iter().collect::<HashSet<String>>()),
            star_points_nodes_set,
            None,
            star_points_node_types_set,
        )
    }

    /// Return vector of tuple of Node IDs that form the edges of the required clique.
    ///
    /// # Arguments
    /// `directed`: Option<bool> - whether to return the edges as directed or undirected. By default, equal to the graph.
    /// `allow_self_loops`: Option<bool> - whether to allow self-loops in the clique. By default, equal to the graph.
    /// `removed_existing_edges`: Option<bool> - whether to filter out the existing edges. By default, true.
    /// `allow_node_type_set`: Option<HashSet<String>> - Node types to include in the clique.
    /// `allow_node_set`: Option<HashSet<String>> - Nodes to include i the clique.
    pub fn get_clique_edges(
        &self,
        directed: Option<bool>,
        allow_self_loops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<Vec<NodeT>> {
        let directed_unwrapped = directed.unwrap_or(self.directed);
        let allow_self_loops_unwrapped = allow_self_loops.unwrap_or_else(|| self.has_selfloops());
        let removed_existing_edges_unwrapped = removed_existing_edges.unwrap_or(true);
        let nodes: Vec<NodeT> = self
            .iter_nodes()
            .filter_map(|(node_id, node_name, _, node_type)| {
                if let (Some(ants), Some(nt)) = (&allow_node_type_set, &node_type) {
                    if nt
                        .iter()
                        .any(|node_type_name| !ants.contains(node_type_name))
                    {
                        return None;
                    }
                }
                if let Some(ans) = &allow_node_set {
                    if !ans.contains(&node_name) {
                        return None;
                    }
                }
                Some(node_id)
            })
            .collect();

        nodes
            .par_iter()
            .flat_map(|src| {
                nodes
                    .iter()
                    .filter_map(|dst| {
                        if !allow_self_loops_unwrapped && src == dst {
                            return None;
                        }
                        if !directed_unwrapped && src > dst {
                            return None;
                        }
                        if removed_existing_edges_unwrapped && self.has_edge_by_node_ids(*src, *dst)
                        {
                            return None;
                        }
                        Some(vec![*src, *dst])
                    })
                    .collect::<Vec<Vec<NodeT>>>()
            })
            .collect()
    }

    /// Return vector of tuple of Node names that form the edges of the required clique.
    ///
    /// # Arguments
    /// `directed`: Option<bool> - whether to return the edges as directed or undirected. By default, equal to the graph.
    /// `allow_self_loops`: Option<bool> - whether to allow self-loops in the clique. By default, equal to the graph.
    /// `removed_existing_edges`: Option<bool> - whether to filter out the existing edges. By default, true.
    /// `allow_node_type_set`: Option<HashSet<String>> - Node types to include in the clique.
    /// `allow_node_set`: Option<HashSet<String>> - Nodes to include i the clique.
    pub fn get_clique_edge_names(
        &self,
        directed: Option<bool>,
        allow_self_loops: Option<bool>,
        removed_existing_edges: Option<bool>,
        allow_node_type_set: Option<HashSet<String>>,
        allow_node_set: Option<HashSet<String>>,
    ) -> Vec<Vec<String>> {
        self.get_clique_edges(
            directed,
            allow_self_loops,
            removed_existing_edges,
            allow_node_type_set,
            allow_node_set,
        )
        .iter()
        .map(|nodes| {
            nodes
                .iter()
                .map(|node| self.get_node_name_by_node_id(*node).unwrap())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
    }
}

use super::*;
use counter::Counter;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::once;

/// # Getters
/// The naming convection we follow is `get_X_by_Y`.
/// The naming convection for unchecked methods follows `get_unchecked_X_by_Y`.
impl Graph {
    /// Return if the graph has any nodes.
    ///
    /// # Example
    /// To check if the graph has nodes you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert_eq!(graph.has_nodes(), true);
    /// ```
    ///
    pub fn has_nodes(&self) -> bool {
        self.get_nodes_number() > 0
    }

    /// Return if the graph has any edges.
    ///
    /// # Example
    /// To check if the current graph has edges you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert_eq!(graph.has_edges(), true);
    /// ```
    ///
    pub fn has_edges(&self) -> bool {
        self.get_edges_number() > 0
    }

    /// Return name of the graph.
    ///
    /// # Example
    /// To the retrieve the name of the current graph instance you can use:
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
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
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("There are {} trap nodes in the current graph.", graph.get_trap_nodes_number());
    /// ```
    ///
    pub fn get_trap_nodes_number(&self) -> EdgeT {
        (self.get_not_singleton_nodes_number() + self.get_singleton_nodes_with_self_loops_number()
            - self.get_unique_source_nodes_number()) as EdgeT
    }

    // Return whether the graph has trap nodes.
    ///
    /// # Example
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// if graph.has_trap_nodes(){
    ///     println!("There are {} trap nodes in the current graph.", graph.get_trap_nodes_number());
    /// } else {
    ///     println!("There are no trap nodes in the current graph.");
    /// }
    /// ```
    ///
    pub fn has_trap_nodes(&self) -> bool {
        self.get_trap_nodes_number() > 0
    }

    /// Returns boolean representing if graph is directed.
    ///
    /// # Example
    /// ```rust
    /// let directed_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(directed_string_ppi.is_directed());
    /// let undirected_string_ppi = graph::test_utilities::load_ppi(true, true, true, false, false, false).unwrap();
    /// assert!(!undirected_string_ppi.is_directed());
    /// ```
    ///
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Returns boolean representing whether graph has weights.
    ///
    /// # Example
    /// ```rust
    /// let weights_string_ppi = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(weights_string_ppi.has_weights());
    /// let unweights_string_ppi = graph::test_utilities::load_ppi(true, true, false, true, false, false).unwrap();
    /// assert!(!unweights_string_ppi.has_weights());
    /// ```
    ///
    pub fn has_weights(&self) -> bool {
        self.weights.is_some()
    }

    /// Returns boolean representing whether graph has edge types.
    ///
    /// # Example
    /// ```rust
    /// let string_ppi_with_edge_types = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(string_ppi_with_edge_types.has_edge_types());
    /// let string_ppi_without_edge_types = graph::test_utilities::load_ppi(true, false, true, true, false, false).unwrap();
    /// assert!(!string_ppi_without_edge_types.has_edge_types());
    /// ```
    ///
    pub fn has_edge_types(&self) -> bool {
        self.edge_types.is_some()
    }

    /// Returns boolean representing if graph has self-loops.
    ///
    /// # Example
    /// ```rust
    /// let string_ppi_with_selfloops = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// assert!(string_ppi_with_selfloops.has_selfloops());
    /// let string_ppi_without_selfloops = graph::test_utilities::load_ppi(true, false, true, true, false, true).unwrap();
    /// assert!(!string_ppi_without_selfloops.has_selfloops());
    /// ```
    ///
    pub fn has_selfloops(&self) -> bool {
        self.self_loop_number > 0
    }

    /// Returns boolean representing if graph has singletons.
    ///
    /// # Example
    /// ```rust
    /// # let graph_with_singletons = graph::test_utilities::load_ppi(true, true, true, false, false, false).unwrap();
    /// assert!(graph_with_singletons.has_singletons());
    /// let graph_without_singletons = graph_with_singletons.remove(
    ///     None, None, None, None, None, None, None, None, false, false, true, true, false, false,
    /// ).unwrap();
    /// assert!(!graph_without_singletons.has_singletons());
    /// ```
    pub fn has_singletons(&self) -> bool {
        self.get_singleton_nodes_number() > 0
    }

    /// Returns boolean representing if graph has singletons.
    pub fn has_singleton_nodes_with_self_loops(&self) -> bool {
        self.get_singleton_nodes_with_self_loops_number() > 0
    }

    /// Return vector of the non-unique source nodes.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_sources(&self, directed: bool) -> Vec<NodeT> {
        self.par_iter_sources_ids(directed).collect()
    }

    /// Return vector of the non-unique source nodes names.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_source_names(&self, directed: bool) -> Vec<String> {
        self.par_iter_sources_ids(directed)
            .map(|src| self.get_node_name_by_node_id(src).unwrap())
            .collect()
    }

    /// Return vector on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_destinations(&self, directed: bool) -> Vec<NodeT> {
        self.par_iter_destinations_ids(directed).collect()
    }

    /// Return vector of the non-unique destination nodes names.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn get_destination_names(&self, directed: bool) -> Vec<String> {
        self.par_iter_destinations_ids(directed)
            .map(|dst| self.get_node_name_by_node_id(dst).unwrap())
            .collect()
    }

    /// Return vector with the sorted nodes names.
    pub fn get_node_names(&self) -> Vec<String> {
        self.nodes.reverse_map.clone()
    }

    /// Return vector with the sorted nodes Ids.
    pub fn get_nodes(&self) -> Vec<NodeT> {
        (0..self.get_nodes_number()).collect()
    }

    /// Return the edge types of the edges.
    pub fn get_edge_types(&self) -> Result<Vec<Option<EdgeTypeT>>, String> {
        if !self.has_edge_types() {
            return Err("The current graph instance does not have edge types!".to_string());
        }
        Ok(self.edge_types.as_ref().map(|ets| ets.ids.clone()).unwrap())
    }

    /// Return the edge types names.
    pub fn get_edge_type_names(&self) -> Option<Vec<String>> {
        self.edge_types
            .as_ref()
            .map(|ets| ets.vocabulary.reverse_map.clone())
    }

    /// Return the node types of the nodes.
    pub fn get_node_types(&self) -> Result<Vec<Option<Vec<NodeTypeT>>>, String> {
        if !self.has_node_types() {
            return Err("The current graph instance does not have nodes!".to_string());
        }
        Ok(self.node_types.as_ref().map(|nts| nts.ids.clone()).unwrap())
    }

    /// Return the weights of the edges.
    pub fn get_weights(&self) -> Result<Vec<WeightT>, String> {
        if !self.has_weights() {
            return Err("The current graph instance does not have weights!".to_string());
        }
        Ok(self.weights.clone().unwrap())
    }

    /// Return the minimum weight, if graph has weights.
    pub fn get_min_weight(&self) -> Result<WeightT, String> {
        self.weights.as_ref().map_or(
            Err("The current graph instance does not have weights!".to_string()),
            |ws| {
                Ok(ws
                    .par_iter()
                    .cloned()
                    .reduce(|| f32::INFINITY, |a, b| a.min(b)))
            },
        )
    }

    /// Return the maximum weight, if graph has weights.
    pub fn get_max_weight(&self) -> Result<WeightT, String> {
        self.weights.as_ref().map_or(
            Err("The current graph instance does not have weights!".to_string()),
            |ws| {
                Ok(ws
                    .par_iter()
                    .cloned()
                    .reduce(|| f32::NEG_INFINITY, |a, b| a.max(b)))
            },
        )
    }

    /// Return the node types names.
    pub fn get_node_type_names(&self) -> Option<Vec<String>> {
        self.node_types
            .as_ref()
            .map(|nts| nts.vocabulary.reverse_map.clone())
    }

    /// Return number of the unique edges in the graph.
    pub fn get_unique_directed_edges_number(&self) -> EdgeT {
        self.unique_edges_number
    }

    /// Return maximum encodable edge number.
    pub fn get_max_encodable_edge_number(&self) -> EdgeT {
        encode_max_edge(
            self.get_nodes_number(),
            get_node_bits(self.get_nodes_number()),
        )
    }

    /// Return the nodes mapping.
    pub fn get_nodes_mapping(&self) -> HashMap<String, NodeT> {
        self.nodes.map.clone()
    }

    /// Return vector with the sorted edge Ids.
    pub fn get_edges(&self, directed: bool) -> Vec<Vec<NodeT>> {
        self.par_iter_edge_ids(directed)
            .map(|(_, src, dst)| vec![src, dst])
            .collect()
    }

    /// Return vector with the sorted edge names.
    pub fn get_edge_names(&self, directed: bool) -> Vec<(String, String)> {
        self.par_iter_edges(directed)
            .map(|(_, _, src_name, _, dst_name)| (src_name, dst_name))
            .collect()
    }

    /// Returns boolean representing if graph has node types.
    pub fn has_node_types(&self) -> bool {
        self.node_types.is_some()
    }

    /// Returns boolean representing if graph has multilabel node types.
    pub fn has_multilabel_node_types(&self) -> bool {
        self.node_types
            .as_ref()
            .map_or(false, |nt| nt.is_multilabel())
    }

    /// Returns number of unknown node types.
    pub fn get_unknown_node_types_number(&self) -> NodeT {
        self.node_types
            .as_ref()
            .map_or(0, |nt| nt.get_unknown_count())
    }

    /// Returns minimum number of node types.
    pub fn get_minimum_node_types_number(&self) -> NodeT {
        self.node_types
            .as_ref()
            .map_or(0, |et| et.min_node_type_count())
    }

    /// Returns whether there are unknown node types.
    pub fn has_unknown_node_types(&self) -> bool {
        self.get_unknown_node_types_number() > 0
    }

    /// Returns number of unknown edge types.
    pub fn get_unknown_edge_types_number(&self) -> EdgeT {
        self.edge_types
            .as_ref()
            .map_or(0, |et| et.get_unknown_count())
    }

    /// Returns minimum number of edge types.
    pub fn get_minimum_edge_types_number(&self) -> EdgeT {
        self.edge_types
            .as_ref()
            .map_or(0, |et| et.min_edge_type_count())
    }

    /// Returns whether there are unknown edge types.
    pub fn has_unknown_edge_types(&self) -> bool {
        self.get_unknown_edge_types_number() > 0
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
    /// * `verbose`: bool - whether to show the loading bar.
    pub fn get_node_components_vector(&self, verbose: bool) -> Vec<NodeT> {
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
    pub fn get_edge_types_number(&self) -> EdgeTypeT {
        self.edge_types
            .as_ref()
            .map_or(0, |ets| ets.len() as EdgeTypeT)
    }

    /// Returns number of node types in the graph.
    pub fn get_node_types_number(&self) -> NodeTypeT {
        self.node_types
            .as_ref()
            .map_or(0, |nts| nts.len() as NodeTypeT)
    }

    /// Returns the degree of every node in the graph.
    pub fn get_node_degrees(&self) -> Vec<NodeT> {
        self.iter_node_degrees().collect()
    }

    /// Return set of nodes that are not singletons.
    pub fn get_not_singletons(&self) -> Vec<NodeT> {
        self.iter_edge_ids(false)
            .flat_map(|(_, src, dst)| once(src).chain(once(dst)))
            .unique()
            .collect()
    }

    /// Return mapping from instance not trap nodes to dense nodes.
    pub fn get_dense_node_mapping(&self) -> HashMap<NodeT, NodeT> {
        self.get_not_singletons()
            .into_iter()
            .enumerate()
            .map(|(i, node)| (node as NodeT, i as NodeT))
            .collect()
    }

    /// Return if there are multiple edges between two nodes
    pub fn is_multigraph(&self) -> bool {
        self.get_multigraph_edges_number() > 0
    }

    /// Return number of edges that have multigraph syblings.
    pub fn get_multigraph_edges_number(&self) -> EdgeT {
        self.get_directed_edges_number() - self.unique_edges_number
    }

    /// Return vector with node degrees
    pub fn get_outbounds(&self) -> Vec<EdgeT> {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|src| self.get_unchecked_edge_id_from_tuple(src as NodeT + 1, 0))
            .collect()
    }

    /// Returns number of the source nodes.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of sources of the graph (not trap nodes) is {}", graph.get_unique_source_nodes_number());
    /// ```
    pub fn get_unique_source_nodes_number(&self) -> NodeT {
        self.unique_sources
            .as_ref()
            .map_or(self.get_nodes_number(), |x| x.len() as NodeT)
    }

    /// Returns edge type counts.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (edge_type_id, count) in graph.get_edge_type_counts().unwrap().iter() {
    ///     println!("edge type id {}: count: {}", edge_type_id, count);
    /// }
    /// ```
    pub fn get_edge_type_counts(&self) -> Result<Counter<EdgeTypeT, usize>, String> {
        if let Some(et) = &self.edge_types {
            Ok(Counter::init(
                et.ids.iter().filter_map(|edge_type| *edge_type),
            ))
        } else {
            Err(String::from(
                "Edge types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns edge type counts hashmap.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (edge_type_id, count) in graph.get_edge_type_counts().unwrap().iter() {
    ///     println!("edge type id {}: count: {}", edge_type_id, count);
    /// }
    /// ```
    pub fn get_edge_type_counts_hashmap(&self) -> Result<HashMap<EdgeTypeT, usize>, String> {
        Ok(self.get_edge_type_counts()?.into_map())
    }

    /// Returns node type counts.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (node_type_id, count) in graph.get_node_type_counts().unwrap().iter() {
    ///     println!("node type id {}: count: {}", node_type_id, count);
    /// }
    /// ```
    pub fn get_node_type_counts(&self) -> Result<Counter<NodeTypeT, usize>, String> {
        if let Some(nt) = &self.node_types {
            Ok(Counter::init(
                nt.ids
                    .iter()
                    .filter_map(|node_type| node_type.clone())
                    .flatten(),
            ))
        } else {
            Err(String::from(
                "Node types are not defined for current graph instance.",
            ))
        }
    }

    /// Returns node type counts hashmap.
    ///
    /// # Arguments
    ///
    /// None
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// for (node_type_id, count) in graph.get_node_type_counts().unwrap().iter() {
    ///     println!("node type id {}: count: {}", node_type_id, count);
    /// }
    /// ```
    pub fn get_node_type_counts_hashmap(&self) -> Result<HashMap<EdgeTypeT, usize>, String> {
        Ok(self.get_node_type_counts()?.into_map())
    }
}

use super::types::*;
use arbitrary::Arbitrary;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Arbitrary)]
pub struct Vocabulary<IndexT: ToFromUsize> {
    pub map: HashMap<String, IndexT>,
    pub reverse_map: Vec<String>,
    pub numeric_ids: bool,
}

impl<IndexT: ToFromUsize> Vocabulary<IndexT> {
    pub fn default() -> Vocabulary<IndexT> {
        Vocabulary {
            map: HashMap::new(),
            reverse_map: Vec::new(),
            numeric_ids: false,
        }
    }

    fn normalize_value(&self, value: &str) -> Result<(String, usize), String> {
        Ok(if self.numeric_ids {
            let parsed_value = match value.parse::<usize>() {
                Ok(val) => Ok(val),
                Err(_) => Err(format!(
                    "The given ID `{}` is not a numeric positive integer.",
                    value
                )),
            }?;

            let string_parsed_value = parsed_value.to_string();

            // Check that there are no extra zeros or separators in the number
            // E.g. 000 is not supported since it will be traduced to 0
            if value != string_parsed_value {
                return Err(format!(
                    concat!(
                        "The given ID is numeric but is not symmetric.\n",
                        "Specifically, {} != {} where the first value is the user's one ",
                        "and the second one is the result of parsing the value as an ",
                        " integer and casting back to string."
                    ),
                    value, string_parsed_value
                ));
            }

            (string_parsed_value, parsed_value)
        } else {
            (value.to_string(), self.map.len())
        })
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub(crate) fn unchecked_insert(&mut self, value: String) -> IndexT {
        let current_length = self.map.len();
        let numeric_ids = self.numeric_ids;
        *self.map.entry(value).or_insert_with_key(|value| {
            IndexT::from_usize(if numeric_ids {
                value.parse::<usize>().unwrap()
            } else {
                current_length
            })
        })
    }

    /// Returns id of given value inserted.
    ///
    /// # Arguments
    ///
    /// * `value`: String - The value to be inserted.
    pub(crate) fn insert<S: AsRef<str>>(&mut self, value: S) -> Result<IndexT, String> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err("The value given to the vocabulary was empty".to_string());
        }

        let (normalized_value, index) = self.normalize_value(value)?;

        if !self.map.contains_key(&normalized_value) {
            self.map
                .insert(normalized_value.clone(), IndexT::from_usize(index));
        }

        Ok(*self.get(&normalized_value).unwrap())
    }

    /// Compute the reverse mapping vector for fast decoding
    pub fn build_reverse_mapping(&mut self) -> Result<(), String> {
        if !self.reverse_map.is_empty() {
            panic!("Build reverse mapping called multiple times!");
        }
        self.reverse_map = vec!["".to_string(); self.map.len()];
        for (k, v) in self.map.iter() {
            if *v >= IndexT::from_usize(self.map.len()) {
                return Err(format!(
                    concat!(
                        "The given set of values is not dense. Found the tuple k:{} v:{} ",
                        "which has index bigger than the number of elements in the map {}."
                    ),
                    k,
                    v,
                    self.map.len()
                ));
            }
            let i = IndexT::to_usize(*v);
            if !self.reverse_map[i].is_empty() {
                return Err(format!(
                    concat!(
                        "During the building of the reverse mapping, ",
                        "one of the elements of the reverse mapping was attempted ",
                        "to be assigned multiple times. This means that in the map ",
                        "there are multiple nodes with the same id.\n",
                        "In the past this was caused by improper handling of numeric ",
                        "node id.\n",
                        "In this case, the value is {} and its index is {}."
                    ),
                    k, i
                ));
            }
            self.reverse_map[i] = k.clone();
        }
        Ok(())
    }

    /// Returns whether the value is empty or not.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn unchecked_translate(&self, id: IndexT) -> String {
        self.reverse_map[IndexT::to_usize(id)].clone()
    }

    /// Returns option with string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: IndexT - Id to be translated.
    pub fn translate(&self, id: IndexT) -> Result<String, String> {
        match self.reverse_map.get(IndexT::to_usize(id)) {
            Some(name) => Ok(name.clone()),
            None => Err("The requested ID is not available in current dictionary.".to_string()),
        }
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<&IndexT> {
        self.map.get(key)
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    /// Return boolean representing if given key is present.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key to check existance of.
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Return length of the vocabulary.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Set whether to load IDs as numeric.
    ///
    /// # Arguments
    /// * numeric_ids: bool - whether to load the IDs as numeric
    ///
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> Vocabulary<IndexT> {
        self.numeric_ids = numeric_ids;
        self
    }
}

use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct NodeTypeVocabulary {
    /// This is the vector with the node types of each node
    /// Moreover, for the node x it's node type is ids[x]
    /// it's an option since the node might not have the node type
    /// and it contains a vector since we support multiple node types
    /// on the same node
    pub ids: Vec<Option<Vec<NodeTypeT>>>,
    pub vocabulary: Vocabulary<NodeTypeT>,
    pub counts: Vec<NodeT>,
    pub unknown_count: NodeT,
    pub multilabel: bool,
}

impl NodeTypeVocabulary {
    fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for NodeTypeVocabulary {
    fn eq(&self, other: &Self) -> bool {
        self.compute_hash() == other.compute_hash()
    }
}

impl NodeTypeVocabulary {
    pub fn default() -> NodeTypeVocabulary {
        NodeTypeVocabulary {
            ids: Vec::new(),
            vocabulary: Vocabulary::default(),
            counts: Vec::new(),
            unknown_count: NodeT::from_usize(0),
            multilabel: false,
        }
    }

    pub fn from_structs(
        ids: Vec<Option<Vec<NodeTypeT>>>,
        vocabulary: Option<Vocabulary<NodeTypeT>>,
    ) -> Option<NodeTypeVocabulary> {
        match vocabulary {
            Some(vocab) => {
                let multilabel = ids
                    .iter()
                    .any(|node_types| node_types.as_ref().map_or(false, |nts| nts.len() > 1));
                let mut vocabvec = NodeTypeVocabulary {
                    ids,
                    vocabulary: vocab,
                    counts: Vec::new(),
                    unknown_count: NodeT::from_usize(0),
                    multilabel,
                };
                vocabvec.build_counts();
                Some(vocabvec)
            }
            None => None,
        }
    }

    pub fn build_counts(&mut self) {
        let mut counts = vec![NodeT::from_usize(0); self.vocabulary.len()];
        for index in self.ids.iter() {
            match index {
                Some(values) => {
                    values.iter().for_each(|value| {
                        counts[NodeTypeT::to_usize(*value)] += NodeT::from_usize(1)
                    });
                }
                None => self.unknown_count += NodeT::from_usize(1),
            }
        }
        self.counts = counts;
    }

    pub fn build_reverse_mapping(&mut self) -> Result<(), String> {
        self.vocabulary.build_reverse_mapping()
    }

    /// Returns ids of given values inserted.
    ///
    /// # Arguments
    ///
    /// * `maybe_values`: Option<Vec<S>> - The values to be inserted.
    pub fn insert_values<S: AsRef<str> + std::fmt::Debug>(
        &mut self,
        maybe_values: Option<Vec<S>>,
    ) -> Result<Option<Vec<NodeTypeT>>, String> {
        Ok(match maybe_values {
            Some(values) => {
                // Check if there is at least one node type
                if values.is_empty() {
                    return Err("The given node types vector is empty.".to_owned());
                }
                // Retrieve the ID
                let mut ids = values
                    .iter()
                    .map(|value| self.vocabulary.insert(value.as_ref()))
                    .collect::<Result<Vec<NodeTypeT>, String>>()?;
                // Sort the slice
                ids.sort_unstable();

                // check for duplicates
                if ids[..ids.len() - 1]
                    .iter()
                    .zip(ids[1..].iter())
                    .any(|(a, b)| a == b)
                {
                    return Err(format!(
                        concat!(
                            "Node with duplicated node types was provided.\n",
                            "Specifically the node types vector of the node is {:?} ",
                        ),
                        values
                    ));
                }
                self.multilabel = self.multilabel || ids.len() > 1;
                // Push the sorted IDs
                self.ids.push(Some(ids.clone()));
                Some(ids)
            }
            None => {
                self.ids.push(None);
                None
            }
        })
    }

    /// Returns whether the vocabulary is empty or not.
    pub fn is_empty(&self) -> bool {
        self.vocabulary.is_empty()
    }

    /// Returns whether the node types are multi-label or not.
    pub fn is_multilabel(&self) -> bool {
        self.multilabel
    }

    /// Returns number of minimum node-count.
    pub fn min_node_type_count(&self) -> NodeT {
        *self.counts.iter().min().unwrap_or(&0)
    }

    /// Returns number of unknown nodes.
    pub fn get_unknown_count(&self) -> NodeT {
        self.unknown_count
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: NodeTypeT - Node Type ID to be translated.
    pub fn unchecked_translate(&self, id: NodeTypeT) -> String {
        self.vocabulary.unchecked_translate(id)
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: NodeTypeT - Node Type ID to be translated.
    pub fn translate(&self, id: NodeTypeT) -> Result<String, String> {
        self.vocabulary.translate(id)
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `ids`: Vec<NodeTypeT> - Node Type IDs to be translated.
    pub fn translate_vector(&self, ids: Vec<NodeTypeT>) -> Result<Vec<String>, String> {
        ids.into_iter().map(|id| self.translate(id)).collect()
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<&NodeTypeT> {
        self.vocabulary.get(key)
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.vocabulary.keys()
    }

    /// Return length of the vocabulary.    
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Set whether to load IDs as numeric.
    ///
    /// # Arguments
    /// * numeric_ids: bool - whether to load the IDs as numeric
    ///
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> NodeTypeVocabulary {
        self.vocabulary = self.vocabulary.set_numeric_ids(numeric_ids);
        self
    }
}

use super::*;

/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
/// * reader: CSVFile - The common reader for reading and writing a csv.
/// * default_node_type: Option<String> - The node type to use if a node has node type or its node type is "".
/// * nodes_column_number: Option<usize> - The rank of the column with the nodes names. This parameter is mutually exclusive with nodes_column.
/// * node_types_separator: Option<String> - Separator to split the node types.
/// * node_types_column_number: Option<usize> - The rank of the column with the nodes types. This parameter is mutually exclusive with node_types_column.
/// * numeric_node_ids: bool - Whether to load the node IDs as numeric.
/// * numeric_node_type_ids: bool - Whether to load the node type IDs as numeric.
/// * skip_node_types_if_unavailable: bool - Whether to skip attempting to load the node types if column is unavailable.
///
#[derive(Clone)]
pub struct NodeFileReader {
    pub(crate) reader: CSVFileReader,
    pub(crate) default_node_type: Option<String>,
    pub(crate) nodes_column_number: Option<usize>,
    pub(crate) node_types_separator: Option<String>,
    pub(crate) node_types_column_number: Option<usize>,
    pub(crate) numeric_node_ids: bool,
    pub(crate) numeric_node_type_ids: bool,
    pub(crate) skip_node_types_if_unavailable: bool,
    pub(crate) might_have_singletons: bool,
}

impl NodeFileReader {
    /// Return new NodeFileReader object.
    ///
    /// # Arguments
    ///
    /// * reader: CSVFileParameters - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> Result<NodeFileReader, String> {
        Ok(NodeFileReader {
            reader: CSVFileReader::new(path, "node list".to_owned())?,
            default_node_type: None,
            nodes_column_number: None,
            node_types_separator: None,
            node_types_column_number: None,
            numeric_node_ids: false,
            numeric_node_type_ids: false,
            skip_node_types_if_unavailable: false,
            might_have_singletons: true,
        })
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_nodes_column<S: Into<String>>(
        mut self,
        nodes_column: Option<S>,
    ) -> Result<NodeFileReader, String> {
        if let Some(column) = nodes_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node column is empty.".to_owned());
            }
            self.nodes_column_number = Some(self.reader.get_column_number(column)?);
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * nodes_column_number: Option<usize> - The nodes column_number to use for the file.
    ///t
    pub fn set_nodes_column_number(mut self, nodes_column_number: Option<usize>) -> NodeFileReader {
        self.nodes_column_number = nodes_column_number;
        self
    }

    /// Set the name of the graph to be loaded.
    ///
    /// # Arguments
    ///
    /// * graph_name: String - The name of the graph to be loaded.
    ///
    pub(crate) fn set_graph_name(mut self, graph_name: String) -> NodeFileReader {
        self.reader.graph_name = graph_name;
        self
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column: Option<String> - The node types column to use for the file.
    ///
    pub fn set_node_types_column<S: Into<String>>(
        mut self,
        nodes_type_column: Option<S>,
    ) -> Result<NodeFileReader, String> {
        if let Some(column) = nodes_type_column {
            let column = column.into();
            if column.is_empty() {
                return Err("The given node types column is empty.".to_owned());
            }
            match self.reader.get_column_number(column) {
                Ok(ecn) => {
                    self.node_types_column_number = Some(ecn);
                }
                Err(e) => {
                    if !self.skip_node_types_if_unavailable {
                        return Err(e);
                    }
                }
            }
        }
        Ok(self)
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * node_types_column_number: Option<usize> - The node types column_number to use for the file.
    ///
    pub fn set_node_types_column_number(
        mut self,
        node_types_column_number: Option<usize>,
    ) -> NodeFileReader {
        self.node_types_column_number = node_types_column_number;
        self
    }

    /// Set whether to automatically skip node_types if they are not avaitable instead of raising an exception.
    ///
    /// # Arguments
    ///
    /// * skip_node_types_if_unavailable: Option<bool> - whether to skip node_types if they are not available.
    ///
    pub fn set_skip_node_types_if_unavailable(
        mut self,
        skip_node_types_if_unavailable: Option<bool>,
    ) -> Result<NodeFileReader, String> {
        if let Some(skip) = skip_node_types_if_unavailable {
            self.skip_node_types_if_unavailable = skip;
        }
        Ok(self)
    }

    /// Set whether you pinky promise that this graph has singletons or not.
    ///
    /// # Arguments
    ///
    /// * might_have_singletons: Option<bool> - Whether this graph has singletons.
    ///
    pub fn set_might_have_singletons(
        mut self,
        might_have_singletons: Option<bool>,
    ) -> Result<NodeFileReader, String> {
        if let Some(skip) = might_have_singletons {
            self.might_have_singletons = skip;
        }
        Ok(self)
    }

    /// Set the comment symbol to use to skip the lines.
    ///
    /// # Arguments
    ///
    /// * comment_symbol: Option<String> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_comment_symbol(
        mut self,
        comment_symbol: Option<String>,
    ) -> Result<NodeFileReader, String> {
        if let Some(cs) = comment_symbol {
            if cs.is_empty() {
                return Err("The given comment symbol is empty.".to_string());
            }
            self.reader.comment_symbol = Some(cs);
        }
        Ok(self)
    }

    /// Set the default node type.
    ///
    /// # Arguments
    ///
    /// * default_node_type: Option<String> - The node type to use when node type is missing.
    ///
    pub fn set_default_node_type<S: Into<String>>(
        mut self,
        default_node_type: Option<S>,
    ) -> NodeFileReader {
        self.default_node_type = default_node_type.map(|val| val.into());
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> NodeFileReader {
        if let Some(v) = verbose {
            self.reader.verbose = v;
        }
        self
    }

    /// Set the numeric_id.
    ///
    /// # Arguments
    ///
    /// * numeric_node_type_ids: Option<bool> - whether to convert numeric node type Ids to Node Type Ids.
    ///
    pub fn set_numeric_node_type_ids(
        mut self,
        numeric_node_type_ids: Option<bool>,
    ) -> NodeFileReader {
        if let Some(nnti) = numeric_node_type_ids {
            self.numeric_node_type_ids = nnti;
        }
        self
    }

    /// Set the numeric_id.
    ///
    /// # Arguments
    ///
    /// * numeric_node_ids: Option<bool> - whether to convert numeric node type Ids to Node Type Ids.
    ///
    pub fn set_numeric_node_ids(mut self, numeric_node_ids: Option<bool>) -> NodeFileReader {
        if let Some(nni) = numeric_node_ids {
            self.numeric_node_ids = nni;
        }
        self
    }

    /// Set the ignore_duplicates.
    ///
    /// # Arguments
    ///
    /// * ignore_duplicates: Option<bool> - whether to ignore detected duplicates or raise exception.
    ///
    pub fn set_ignore_duplicates(mut self, ignore_duplicates: Option<bool>) -> NodeFileReader {
        if let Some(v) = ignore_duplicates {
            self.reader.ignore_duplicates = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(
        mut self,
        separator: Option<S>,
    ) -> Result<NodeFileReader, String> {
        if let Some(sep) = separator {
            let sep = sep.into();
            if sep.is_empty() {
                return Err("The separator cannot be empty.".to_owned());
            }
            self.reader.separator = sep;
        }
        Ok(self)
    }

    /// Set the node types separator.
    ///
    /// In the following example we show a column of node IDs and
    /// a column of node types.
    ///
    /// ```bash
    /// node_id_columns node_types
    /// node_A node_type_1|node_type_2
    /// node_B node_type_2
    /// ```  
    ///
    /// # Arguments
    ///
    /// * node_types_separator: Option<String> - The separator to use for the node types column.
    ///
    pub fn set_node_types_separator<S: Into<String>>(
        mut self,
        node_types_separator: Option<S>,
    ) -> Result<NodeFileReader, String> {
        if let Some(sep) = node_types_separator {
            let sep = sep.into();
            if sep.is_empty() {
                return Err("The node type separator cannot be empty.".to_owned());
            }
            self.node_types_separator = Some(sep);
        }
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> NodeFileReader {
        if let Some(v) = header {
            self.reader.header = v;
        }
        self
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> NodeFileReader {
        if let Some(v) = rows_to_skip {
            self.reader.rows_to_skip = v;
        }
        self
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    ///
    /// * max_rows_number: Option<u64> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(mut self, max_rows_number: Option<u64>) -> NodeFileReader {
        self.reader.max_rows_number = max_rows_number;
        self
    }

    /// Return boolean representing if the node types exist.
    pub fn has_node_types(&self) -> bool {
        self.default_node_type.is_some() || self.node_types_column_number.is_some()
    }

    /// Return iterator of the lines of the node file.
    pub fn read_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + '_, String>
    {
        // Validating that at least a column was given.
        if [self.nodes_column_number, self.node_types_column_number]
            .iter()
            .all(|val| val.is_none())
        {
            return Err("Neither nodes ID column or node types column were given!".to_string());
        }

        // Check that the two columns do not have the same value.
        if self.nodes_column_number == self.node_types_column_number {
            return Err("The node column is the same as the node type one.".to_string());
        }

        // Retrieve the expected maximum number of columns.
        let expected_number_of_elements = self.reader.get_elements_per_line()?;

        // Check that the two columns do not have a value higher than the maximum amount.
        for column in [self.nodes_column_number, self.node_types_column_number]
            .iter()
            .filter_map(|maybe_column| *maybe_column)
        {
            if column >= expected_number_of_elements {
                return Err(format!(
                    concat!(
                        "A column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_number_of_elements
                ));
            }
        }

        Ok(self
            .reader
            .read_lines()?
            .enumerate()
            .map(move |(line_number, values)| match values {
                Ok(vals) => {
                    let node_name = match self.nodes_column_number {
                        Some(column) => match vals[column].to_owned() {
                            Some(node_name) => node_name,
                            None => {
                                return Err(
                                    "One of the provided node IDs is empty or None.".to_owned()
                                )
                            }
                        },
                        None => line_number.to_string(),
                    };
                    let maybe_node_types_string = match self.node_types_column_number {
                        Some(column) => match vals[column].to_owned() {
                            Some(node_type) => Some(node_type),
                            None => self.default_node_type.clone(),
                        },
                        None => self.default_node_type.clone(),
                    };

                    // Split given node types using the provided node type separator.
                    let node_types = match maybe_node_types_string {
                        Some(string) => match &self.node_types_separator {
                            Some(sep) => Some(string.split(sep).map(String::from).collect()),
                            None => Some(vec![string]),
                        },
                        None => None,
                    };

                    // Return tuple with string and list of node types
                    Ok((node_name, node_types))
                }
                Err(e) => Err(e),
            }))
    }
}

use super::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl EdgeTypeVocabulary {
    fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for EdgeTypeVocabulary {
    fn eq(&self, other: &Self) -> bool {
        self.compute_hash() == other.compute_hash()
    }
}

#[derive(Debug, Clone)]
pub struct EdgeTypeVocabulary {
    pub ids: Vec<Option<EdgeTypeT>>,
    pub vocabulary: Vocabulary<EdgeTypeT>,
    pub counts: Vec<EdgeT>,
    pub unknown_count: EdgeT,
}

impl EdgeTypeVocabulary {
    pub fn default() -> EdgeTypeVocabulary {
        EdgeTypeVocabulary {
            ids: Vec::new(),
            vocabulary: Vocabulary::default(),
            counts: Vec::new(),
            unknown_count: EdgeT::from_usize(0),
        }
    }

    pub fn from_structs(
        ids: Vec<Option<EdgeTypeT>>,
        vocabulary: Vocabulary<EdgeTypeT>,
    ) -> EdgeTypeVocabulary {
        let mut vocabvec = EdgeTypeVocabulary {
            ids,
            vocabulary,
            counts: Vec::new(),
            unknown_count: EdgeT::from_usize(0),
        };

        vocabvec.build_counts();

        vocabvec
    }

    pub fn from_option_structs(
        ids: Option<Vec<Option<EdgeTypeT>>>,
        vocabulary: Option<Vocabulary<EdgeTypeT>>,
    ) -> Option<EdgeTypeVocabulary> {
        if let (Some(ids), Some(vocabulary)) = (ids, vocabulary) {
            Some(EdgeTypeVocabulary::from_structs(ids, vocabulary))
        } else {
            None
        }
    }

    pub fn build_counts(&mut self) {
        self.counts = vec![EdgeT::from_usize(0); self.vocabulary.len()];
        for index in self.ids.iter() {
            match index {
                Some(value) => {
                    self.counts[*value as usize] += 1;
                }
                None => self.unknown_count += EdgeT::from_usize(1),
            }
        }
    }

    /// Returns whether the value is empty or not.
    pub fn is_empty(&self) -> bool {
        self.vocabulary.is_empty()
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: EdgeTypeT - Id to be translated.
    pub fn unchecked_translate(&self, id: EdgeTypeT) -> String {
        self.vocabulary.unchecked_translate(id)
    }

    /// Returns string name of given id.
    ///
    /// # Arguments
    ///
    /// * `id`: EdgeTypeT - Id to be translated.
    pub fn translate(&self, id: EdgeTypeT) -> Result<String, String> {
        self.vocabulary.translate(id)
    }

    /// Return the id of given key.
    ///
    /// # Arguments
    ///
    /// * `key`: &str - the key whose Id is to be retrieved.
    pub fn get(&self, key: &str) -> Option<&EdgeTypeT> {
        self.vocabulary.get(key)
    }

    /// Return vector of keys of the map.
    pub fn keys(&self) -> Vec<String> {
        self.vocabulary.keys()
    }

    /// Return length of the vocabulary.    
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Set whether to load IDs as numeric.
    ///
    /// # Arguments
    /// * numeric_ids: bool - whether to load the IDs as numeric
    ///
    pub fn set_numeric_ids(mut self, numeric_ids: bool) -> EdgeTypeVocabulary {
        self.vocabulary = self.vocabulary.set_numeric_ids(numeric_ids);
        self
    }

    /// Returns number of unknown edges.
    pub fn get_unknown_count(&self) -> EdgeT {
        self.unknown_count
    }

    /// Returns number of minimum edge-count.
    pub fn min_edge_type_count(&self) -> EdgeT {
        *self.counts.iter().min().unwrap_or(&0)
    }
}

use super::*;
use rayon::prelude::*;

/// # Iterators
/// The naming convention for the iterators is:
/// If the method has the `par_` prefix then it should return a parallel iterator.
/// By default all the methods retruns both the ids and the name of the item and
/// if the method has the suffix `_ids` then it will returns **only** the ids.
impl Graph {
    /// Returns range of the edge ids of edges starting from the given source node.
    ///
    /// # Arguments
    ///
    /// * `src` - Source node of the edge.
    ///
    pub(crate) fn iter_unchecked_edge_ids_by_source_node_id(
        &self,
        src: NodeT,
    ) -> std::ops::Range<usize> {
        let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_by_source_node_id(src);
        min_edge_id as usize..max_edge_id as usize
    }

    /// Return iterator on the node degrees of the graph.
    pub fn iter_node_degrees(&self) -> impl Iterator<Item = NodeT> + '_ {
        (0..self.get_nodes_number()).map(move |node| self.get_node_degree_by_node_id(node).unwrap())
    }

    /// Return iterator on the node degrees of the graph.
    pub fn par_iter_node_degrees(&self) -> impl ParallelIterator<Item = NodeT> + '_ {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(move |node| self.get_node_degree_by_node_id(node).unwrap())
    }

    /// Return iterator over NodeT of destinations of the given node src.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbours are to be retrieved.
    ///
    pub(crate) fn iter_node_neighbours_ids(&self, src: NodeT) -> Box<dyn Iterator<Item = NodeT> + '_> {
        match &self.destinations{
            Some(dsts) => {
                Box::new(dsts[self.iter_unchecked_edge_ids_by_source_node_id(src)].iter().cloned())
            },
            None => Box::new(self.edges
                .iter_in_range(self.encode_edge(src, 0)..self.encode_edge(src + 1, 0))
                .map(move |edge| self.decode_edge(edge).1))
        }
    }

    /// Return iterator over NodeT of destinations of the given node src.
    ///
    /// # Arguments
    /// * `src`: NodeT - The node whose neighbour names are to be retrieved.
    ///
    pub(crate) fn iter_node_neighbours(&self, src: NodeT) -> impl Iterator<Item = String> + '_ {
        self.iter_node_neighbours_ids(src)
            .map(move |dst| {
                self.get_unchecked_node_name_by_node_id(dst)
            })
    }

    /// Return iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_sources_ids(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_edge_ids(directed).map(move |(_, src, _)| src)
    }

    /// Return parallel iterator on the (non unique) source nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_sources_ids(&self, directed: bool) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_edge_ids(directed).map(move |(_, src, _)| src)
    }

    /// Return iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_destinations_ids(&self, directed: bool) -> impl Iterator<Item = NodeT> + '_ {
        self.iter_edge_ids(directed).map(move |(_, _, dst)| dst)
    }

    /// Return parallel iterator on the (non unique) destination nodes of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_destinations_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = NodeT> + '_ {
        self.par_iter_edge_ids(directed).map(move |(_, _, dst)| dst)
    }

    /// Return iterator on the node of the graph.
    pub fn iter_node_ids(&self) -> impl Iterator<Item = (NodeT, Option<Vec<NodeTypeT>>)> + '_ {
        (0..self.get_nodes_number())
            .map(move |node_id| (node_id, self.get_unchecked_node_type_id_by_node_id(node_id)))
    }

    /// Return iterator on the node of the graph as Strings.
    pub fn iter_nodes(
        &self,
    ) -> impl Iterator<Item = (NodeT, String, Option<Vec<NodeTypeT>>, Option<Vec<String>>)> + '_
    {
        self.iter_node_ids().map(move |(node_id, node_types)| {
            (
                node_id,
                self.nodes.unchecked_translate(node_id),
                node_types,
                self.get_node_type_name_by_node_id(node_id).unwrap_or(None),
            )
        })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edge_ids(
        &self,
        directed: bool,
    ) -> Box<dyn Iterator<Item = (EdgeT, NodeT, NodeT)> + '_> {
        if self.sources.is_some() && self.destinations.is_some() {
            return Box::new(
                (0..self.get_directed_edges_number()).filter_map(move |edge_id| {
                    let (src, dst) = self.get_node_ids_from_edge_id(edge_id);
                    if !directed && src > dst {
                        return None;
                    }
                    Some((edge_id, src, dst))
                }),
            );
        }
        Box::new(
            self.edges
                .iter()
                .enumerate()
                .filter_map(move |(edge_id, edge)| {
                    let (src, dst) = self.decode_edge(edge);
                    if !directed && src > dst {
                        return None;
                    }
                    Some((edge_id as EdgeT, src, dst))
                }),
        )
    }
    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edges(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, String, NodeT, String)> + '_ {
        self.iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    self.get_unchecked_node_name_by_node_id(src),
                    dst,
                    self.get_unchecked_node_name_by_node_id(dst),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT)> + '_ {
        self.edges
            .par_enumerate()
            .filter_map(move |(edge_id, edge)| {
                let (src, dst) = self.decode_edge(edge);
                if !directed && src > dst {
                    return None;
                }
                Some((edge_id as EdgeT, src, dst))
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edges(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, String, NodeT, String)> + '_ {
        self.par_iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    self.get_unchecked_node_name_by_node_id(src),
                    dst,
                    self.get_unchecked_node_name_by_node_id(dst),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edges_with_type_ids(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_by_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edges_with_type(
        &self,
        directed: bool,
    ) -> impl Iterator<
        Item = (
            EdgeT,
            NodeT,
            String,
            NodeT,
            String,
            Option<EdgeTypeT>,
            Option<String>,
        ),
    > + '_ {
        self.iter_edges(directed)
            .map(move |(edge_id, src, src_name, dst, dst_name)| {
                let edge_type_id = self.get_unchecked_edge_type_by_edge_id(edge_id);
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type_id,
                    self.get_unchecked_edge_type_name_by_edge_type_id(edge_type_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the ids and string name.
    /// The result is (edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_with_type(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<
        Item = (
            EdgeT,
            NodeT,
            String,
            NodeT,
            String,
            Option<EdgeTypeT>,
            Option<String>,
        ),
    > + '_ {
        self.par_iter_edges(directed)
            .map(move |(edge_id, src, src_name, dst, dst_name)| {
                let edge_type_id = self.get_unchecked_edge_type_by_edge_id(edge_id);
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type_id,
                    self.get_unchecked_edge_type_name_by_edge_type_id(edge_type_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_with_type_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>)> + '_ {
        self.par_iter_edge_ids(directed)
            .map(move |(edge_id, src, dst)| {
                (
                    edge_id,
                    src,
                    dst,
                    self.get_unchecked_edge_type_by_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_with_type_and_weight(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<
        Item = (
            EdgeT,
            NodeT,
            String,
            NodeT,
            String,
            Option<EdgeTypeT>,
            Option<String>,
            Option<WeightT>,
        ),
    > + '_ {
        self.par_iter_edge_with_type(directed).map(
            move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| {
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type,
                    edge_type_name,
                    self.get_unchecked_weight_by_edge_id(edge_id),
                )
            },
        )
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edge_with_type_and_weight(
        &self,
        directed: bool,
    ) -> impl Iterator<
        Item = (
            EdgeT,
            NodeT,
            String,
            NodeT,
            String,
            Option<EdgeTypeT>,
            Option<String>,
            Option<WeightT>,
        ),
    > + '_ {
        self.iter_edges_with_type(directed).map(
            move |(edge_id, src, src_name, dst, dst_name, edge_type, edge_type_name)| {
                (
                    edge_id,
                    src,
                    src_name,
                    dst,
                    dst_name,
                    edge_type,
                    edge_type_name,
                    self.get_unchecked_weight_by_edge_id(edge_id),
                )
            },
        )
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn par_iter_edge_with_type_and_weight_ids(
        &self,
        directed: bool,
    ) -> impl ParallelIterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_
    {
        self.par_iter_edge_with_type_ids(directed)
            .map(move |(edge_id, src, dst, edge_type)| {
                (
                    edge_id,
                    src,
                    dst,
                    edge_type,
                    self.get_unchecked_weight_by_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph with the string name.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_edge_with_type_and_weight_ids(
        &self,
        directed: bool,
    ) -> impl Iterator<Item = (EdgeT, NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>)> + '_ {
        self.iter_edges_with_type_ids(directed)
            .map(move |(edge_id, src, dst, edge_type)| {
                (
                    edge_id,
                    src,
                    dst,
                    edge_type,
                    self.get_unchecked_weight_by_edge_id(edge_id),
                )
            })
    }

    /// Return iterator on the edges of the graph.
    ///
    /// # Arguments
    /// * `directed`: bool, whether to filter out the undirected edges.
    pub fn iter_unique_edges(
        &self,
        directed: bool,
    ) -> Box<dyn Iterator<Item = (NodeT, NodeT)> + '_> {
        if self.sources.is_some() && self.destinations.is_some() {
            return Box::new(
                (0..self.get_directed_edges_number()).filter_map(move |edge_id| {
                    let (src, dst) = self.get_node_ids_from_edge_id(edge_id);
                    if edge_id > 0 {
                        let (last_src, last_dst) = self.get_node_ids_from_edge_id(edge_id - 1);
                        if last_src == src && last_dst == dst {
                            return None;
                        }
                    }
                    if !directed && src > dst {
                        return None;
                    }
                    Some((src, dst))
                }),
            );
        }
        Box::new(self.edges.iter_uniques().filter_map(move |edge| {
            let (src, dst) = self.decode_edge(edge);
            if !directed && src > dst {
                return None;
            }
            Some((src, dst))
        }))
    }

    /// Returns option of range of multigraph minimum and maximum edge ids with same source and destination nodes and different edge type.
    ///
    /// # Arguments
    ///
    /// * `src` - Source node id of the edge.
    /// * `dst` - Destination node id of the edge.
    ///
    pub(crate) fn iter_edge_ids_by_node_ids(
        &self,
        src: NodeT,
        dst: NodeT,
    ) -> Option<impl Iterator<Item = EdgeT>> {
        self.get_minmax_edge_ids_by_node_ids(src, dst)
            .map(|(min_edge_id, max_edge_id)| min_edge_id..max_edge_id)
    }

    /// Return iterator on the unique sources of the graph.
    pub fn iter_unique_sources(&self) -> Box<dyn Iterator<Item = NodeT> + '_> {
        if let Some(x) = &self.unique_sources {
            return Box::new(x.iter().map(|source| source as NodeT));
        }
        Box::new(0..self.get_nodes_number())
    }
}

use super::*;

impl Graph {
    /// Return graph renderized from given files.
    ///
    /// # Arguments
    ///
    /// * `edge_file_reader`: EdgeFileReader - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `directed`: bool - whether the graph is to be read as directed or undirected.
    /// * `directed_edge_list`: bool - whether to read the edge list as directed.
    /// * `edges_number`: usize - Number of edges of the graph.
    /// * `nodes_number`: NodeT - Number of the nodes of the graph.
    /// * `name`: S - Name of the graph.
    ///
    pub fn from_sorted_csv<S: Clone + Into<String>>(
        mut edge_file_reader: EdgeFileReader,
        mut node_file_reader: Option<NodeFileReader>,
        directed: bool,
        directed_edge_list: bool,
        edges_number: usize,
        nodes_number: NodeT,
        name: S,
    ) -> Result<Graph, String> {
        edge_file_reader = edge_file_reader.set_graph_name(name.clone().into());
        node_file_reader = node_file_reader.map(|nfr| nfr.set_graph_name(name.clone().into()));
        Graph::from_string_sorted(
            edge_file_reader.read_lines()?,
            node_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |nfr| Ok(Some(nfr.read_lines()?)))?,
            directed,
            directed_edge_list,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.ignore_duplicates),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.csv_is_correct),
            edge_file_reader.reader.ignore_duplicates,
            edge_file_reader.reader.csv_is_correct,
            edges_number,
            nodes_number,
            edge_file_reader.numeric_edge_type_ids,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_ids),
            edge_file_reader.numeric_node_ids,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_type_ids),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.has_node_types()),
            edge_file_reader.has_edge_types(),
            edge_file_reader.has_weights(),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.might_have_singletons),
            edge_file_reader.might_have_singletons_with_selfloops,
            edge_file_reader.might_have_trap_nodes,
            name,
        )
    }

    /// Return graph renderized from given files.
    ///
    /// # Arguments
    ///
    /// * `edge_file_reader`: EdgeFileReader - Reader of the edge file.
    /// * `node_file_reader`: Option<NodeFileReader> - Reader of the node file.
    /// * `directed`: bool - whether the graph is to be read as directed or undirected.
    /// * `directed_edge_list`: bool - whether to read the edge list as directed.
    pub fn from_unsorted_csv<S: Clone + Into<String>>(
        mut edge_file_reader: EdgeFileReader,
        mut node_file_reader: Option<NodeFileReader>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
    ) -> Result<Graph, String> {
        edge_file_reader = edge_file_reader.set_graph_name(name.clone().into());
        node_file_reader = node_file_reader.map(|nfr| nfr.set_graph_name(name.clone().into()));
        Graph::from_string_unsorted(
            edge_file_reader.read_lines()?,
            node_file_reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |nfr| Ok(Some(nfr.read_lines()?)))?,
            directed,
            directed_edge_list,
            name,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.ignore_duplicates),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.reader.csv_is_correct),
            edge_file_reader.reader.ignore_duplicates,
            edge_file_reader.reader.csv_is_correct,
            edge_file_reader.reader.verbose,
            edge_file_reader.numeric_edge_type_ids,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_ids),
            edge_file_reader.numeric_node_ids,
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.numeric_node_type_ids),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.has_node_types()),
            edge_file_reader.has_edge_types(),
            edge_file_reader.has_weights(),
            node_file_reader
                .as_ref()
                .map_or(false, |nfr| nfr.might_have_singletons),
            edge_file_reader.might_have_singletons_with_selfloops,
            edge_file_reader.might_have_trap_nodes,
        )
    }
}

//! Ensmallen its an efficient graph manipulation library.
//!
//! # Example:
//!
//! ```rust
//! use graph::{EdgeFileReader, Graph};
//! let edges_reader = EdgeFileReader::new("tests/data/test_components.csv",).unwrap()
//!     .set_separator(Some(",")).unwrap()
//!     .set_verbose(Some(false))
//!     .set_numeric_node_ids(Some(true))
//!     .set_header(Some(false));
//!  
//! let g = Graph::from_sorted_csv(edges_reader, None, false, false, 6108, 242, "Graph").unwrap();
//!
//!
//! ```
#![warn(unused_macros)]
#![feature(map_first_last)]
#![type_length_limit = "3764086"]

const SEED_XOR: usize = 0xbad5eedbad5eed11;

mod vocabulary;
pub use self::vocabulary::Vocabulary;
mod node_type_vocabulary;
pub use self::node_type_vocabulary::NodeTypeVocabulary;
mod edge_type_vocabulary;
pub use self::edge_type_vocabulary::EdgeTypeVocabulary;

mod csv_file_writer;
pub(crate) use self::csv_file_writer::compose_lines;
pub use self::csv_file_writer::CSVFileWriter;
mod csv_file_reader;
pub use self::csv_file_reader::CSVFileReader;
mod node_file_reader;
pub use self::node_file_reader::NodeFileReader;
mod node_file_writer;
pub use self::node_file_writer::NodeFileWriter;
mod edge_file_reader;
pub use self::edge_file_reader::EdgeFileReader;
mod edge_file_writer;
pub use self::edge_file_writer::EdgeFileWriter;
mod compression;
mod from_csv;
pub(crate) use self::compression::*;


mod constructors;

pub mod utils;
pub(crate) use self::utils::*;

mod bitmaps;
mod edge_lists;
mod filters;
mod getters;
mod graph;
mod hash;
mod holdouts;
mod iters;
mod metrics;
mod modifiers;
mod operators;
mod preprocessing;
mod remap;
mod remove;
mod setters;
mod tarjan;
mod trees;
mod types;
mod walks;
pub mod walks_parameters;

mod queries;
mod queries_boolean;
mod queries_unchecked;
mod queries_walk;
pub use self::queries::*;
pub use self::queries_boolean::*;

pub mod test_utilities;

pub use self::getters::*;
pub use self::graph::Graph;
pub use self::holdouts::*;
pub use self::metrics::*;
pub use self::operators::*;
pub use self::setters::*;
pub use self::tarjan::*;
pub use self::trees::*;
pub use self::types::*;
pub use self::walks::*;
pub use self::walks_parameters::*;
pub use preprocessing::*;

use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[inline(always)]
/// Hashing floats is usually a bad idea
/// But we want to know if any weight changed significantly
/// THUS we will hash only the order of magnitude and the
/// first few bits of the mantissa.
///
/// This should be an hash which is kinda robust to float erros.
fn hash_float<H: Hasher>(x: f32, state: &mut H) {
    // basically we are converting the float to a u32 and
    // clear out the lower bits of the mantissa.
    let mut hack = u32::from_le_bytes(x.to_le_bytes());

    // Clear the lower bits of the mantissa
    //        seeeeeeeemmmmmmmmmmmmmmmmmmmmmmm
    hack &= 0b11111111111111111111000000000000;

    state.write_u32(hack);
}

impl Graph {
    pub fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.compute_hash() == other.compute_hash()
    }
}

impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // These fields are fundamentals
        self.directed.hash(state);
        self.nodes.hash(state);
        self.edges.hash(state);

        if let Some(ws) = &self.weights {
            for w in ws {
                hash_float(*w, state);
            }
        }

        if let Some(nt) = &self.node_types {
            nt.hash(state);
        }

        if let Some(et) = &self.edge_types {
            et.hash(state);
        }

        // These fields are derivative from the other ones and thus not needed.
        // self.unique_sources.hash(state);
        // self.node_bits.hash(state);
        // self.node_bit_mask.hash(state);
        // self.unique_self_loop_number.hash(state);
        // self.self_loop_number.hash(state);
        // self.not_singleton_nodes_number.hash(state);
        // self.singleton_nodes_with_self_loops_number.hash(state);
        // self.unique_edges_number.hash(state);

        // These fields are not meaningfull to hash imho
        // self.name.hash(state);
        // self.singleton_nodes_with_self_loops_number.hash(state);
        // self.sources.hash(state);
        // self.outbounds.hash(state);
        // self.cached_destinations.hash(state);
        // self.embedding.hash(state);
    }
}

impl<IndexT: ToFromUsize> Hash for Vocabulary<IndexT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // The hashmap is not hashable, so we convert it to a
        // sorted array of tuples.

        let mut vector: Vec<(&String, &IndexT)> = self.map.iter().collect();
        vector.sort();
        vector.hash(state);

        self.reverse_map.hash(state);
        self.numeric_ids.hash(state);
    }
}

impl Hash for NodeTypeVocabulary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ids.hash(state);
        self.vocabulary.hash(state);
        self.counts.hash(state);
    }
}

impl Hash for EdgeTypeVocabulary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ids.hash(state);
        self.vocabulary.hash(state);
        self.counts.hash(state);
    }
}

use super::*;
use bitvec::prelude::*;
use elias_fano_rust::EliasFano;
use indicatif::ProgressIterator;
use itertools::Itertools;
use log::info;
use rayon::prelude::ParallelSliceMut;
use roaring::RoaringBitmap;
use std::cmp::Ordering;
use std::collections::BTreeMap;

type ParsedStringEdgesType = Result<
    (
        EliasFano,
        Option<EliasFano>,
        Vocabulary<NodeT>,
        Option<EdgeTypeVocabulary>,
        Option<Vec<WeightT>>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u64,
        u8,
        Option<BitVec<Lsb0, u8>>,
        Option<RoaringBitmap>,
    ),
    String,
>;

#[macro_export]
/// Take a vector and make it a None if its empty, Some(vector) otherwise
macro_rules! optionify {
    ($val:expr) => {
        if $val.is_empty() {
            None
        } else {
            Some($val)
        }
    };
}

fn check_numeric_ids_compatibility(
    has_nodes_list: bool,
    numeric_node_ids: bool,
    numeric_edge_node_ids: bool,
) -> Result<(), String> {
    if has_nodes_list && numeric_node_ids && !numeric_edge_node_ids {
        return Err(concat!(
            "You are trying to load a numeric node list and a non numeric edge list.\n",
            "This is a problem because an edge composed of two nodes (e.g. \"2, 8\") is ",
            "not necessarily mapped internally to the same node ids of the node list.\n",
            "Possibily you want to also enable the parameter for the numeric edge node ids."
        )
        .to_string());
    }
    Ok(())
}

/// Returns iterator of nodes handling the node IDs.
///
/// # Arguments
///
/// nodes_iter: impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + 'a,
///     Iterator over the node list.
/// ignore_duplicated_nodes: bool,
///     Whether to just ignore the duplicated node types.
/// node_list_is_correct: bool,
///     Parameter to pinky promise that the node list is correct.
///     If you provide a broken node list to this method while promising
///     that the node list is correct, be prepared to deal with the fallout.
///     This parameter is mainly meant to be used internally when creating
///     graphs that CANNOT BE BROKEN by design. If you use this parameter
///     from any of the bindings, be SURE that the node list is actually
///     correct.
///     We assume that any provided node list is broken until disproved.
/// nodes: &'b mut Vocabulary<NodeT>,
///     Vocabulary of the nodes to be populated.
pub(crate) fn parse_node_ids<'a, 'b>(
    nodes_iter: impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + 'a,
    ignore_duplicated_nodes: bool,
    node_list_is_correct: bool,
    nodes: &'b mut Vocabulary<NodeT>,
) -> impl Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a
where
    'b: 'a,
{
    nodes_iter.filter_map(move |row| {
        row.map_or_else(|err| Some(Err(err)), |(node_name, node_type)| {
            if node_list_is_correct {
                Some(Ok((nodes.unchecked_insert(node_name), node_type)))
            } else {
                if node_name.is_empty() {
                    return Some(Err("Found an empty node name. Node names cannot be empty.".to_owned()));
                }
                if nodes.contains_key(&node_name){
                    if ignore_duplicated_nodes {
                        return None;
                    }
                    return Some(Err(format!(
                        concat!(
                            "The node {node_name} appears multiple times in the node list.\n",
                            "The node type of the row is {node_type:?}.\n",
                            "The library does not currently support multiple node types for a single node."
                        ),
                        node_name = node_name,
                        node_type = node_type
                    )));
                }
                Some(nodes.insert(node_name).map(|node_id| (node_id, node_type)))
            }
        })
    })
}

/// Returns iterator of nodes handling the node type IDs.
pub(crate) fn parse_node_type_ids<'a, 'b>(
    nodes_iter: impl Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a,
    node_types_vocabulary: &'b mut NodeTypeVocabulary,
) -> impl Iterator<Item = Result<(NodeT, Option<Vec<NodeTypeT>>), String>> + 'a
where
    'b: 'a,
{
    nodes_iter.map(move |row| match row {
        Ok((node_id, node_types)) => {
            Ok((node_id, node_types_vocabulary.insert_values(node_types)?))
        }
        Err(e) => Err(e),
    })
}

pub(crate) fn parse_edges_node_ids<'a, 'b>(
    edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>> + 'a,
    edge_list_is_correct: bool,
    nodes: &'b mut Vocabulary<NodeT>,
) -> impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>> + 'a
where
    'b: 'a,
{
    let empty_nodes_mapping = nodes.is_empty();
    edges_iterator.map(move |row: Result<StringQuadruple, String>| match row {
        Ok((src_name, dst_name, edge_type, weight)) => {
            let node_ids = [src_name, dst_name]
                .iter()
                .map(|node_name| {
                    // the source and destination nodes must either be
                    //  - both numeric node ids
                    //      - if the node list was provided
                    //          - The nodes must be less than the max nodes
                    //      - if the node list was not provided
                    //          - the nodes must be added to the node list which should be numeric.
                    //  - if the edge node ids are not numeric
                    //      - if the node list was provided
                    //          - the nodes must be added to the node list.
                    //      - if the node list was no provided
                    //          - the nodes must be added to the node list.
                    if empty_nodes_mapping {
                        if edge_list_is_correct {
                            Ok(nodes.unchecked_insert(node_name.to_owned()))
                        } else {
                            nodes.insert(node_name.to_owned())
                        }
                    } else if let Some(node_id) = nodes.get(&node_name) {
                        Ok(*node_id)
                    } else {
                        Err(format!(
                            concat!(
                                "In the edge list was found the node {} ",
                                "which is not present in the given node list."
                            ),
                            node_name
                        ))
                    }
                })
                .collect::<Result<Vec<NodeT>, String>>()?;
            Ok((node_ids[0], node_ids[1], edge_type, weight))
        }
        Err(e) => Err(e),
    })
}

/// Returns iterator of edges handling the edge type IDs.
pub(crate) fn parse_edge_type_ids_vocabulary<'a, 'b>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>>
        + 'a,
    edge_types: &'b mut Vocabulary<EdgeTypeT>,
) -> impl Iterator<Item = Result<Quadruple, String>> + 'a
where
    'b: 'a,
{
    edges_iter.map(move |row| match row {
        Ok((src, dst, edge_type, weight)) => {
            let edge_type_id = match edge_type {
                Some(et) => Some(edge_types.insert(et)?),
                None => None,
            };
            Ok((src, dst, edge_type_id, weight))
        }
        Err(e) => Err(e),
    })
}

pub(crate) fn parse_sorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>> + 'a,
    directed: bool,
    directed_edge_list: bool,
) -> Box<dyn Iterator<Item = Result<Quadruple, String>> + 'a> {
    if directed || directed_edge_list {
        return Box::new(edges_iter);
    }
    let mut sorting_tmp: BTreeMap<Triple, Option<WeightT>> = BTreeMap::new();
    Box::new(edges_iter
        .map(Some)
        .chain(vec![None])
        .flat_map(move |maybe_row| match maybe_row {
            Some(row) => {
                let mut results: Vec<Result<Quadruple, String>> = Vec::with_capacity(1);
                let result = match row {
                    Ok((src, dst, edge_type, weight)) => {
                        if !directed && src < dst {
                            sorting_tmp.insert((dst, src, edge_type), weight);
                        }
                        while !sorting_tmp.is_empty()
                            && *sorting_tmp.first_key_value().unwrap().0 < (src, dst, edge_type)
                        {
                            let ((smaller_src, smaller_dst, smaller_edge_type), smaller_weight) =
                                sorting_tmp.pop_first().unwrap();
                            results.push(Ok((
                                smaller_src,
                                smaller_dst,
                                smaller_edge_type,
                                smaller_weight,
                            )));
                        }
                        Ok((src, dst, edge_type, weight))
                    }
                    Err(e) => Err(e),
                };
                results.push(result);
                results
            }
            None => sorting_tmp
                .iter()
                .map(|((src, dst, edge_type), weight)| Ok((*src, *dst, *edge_type, *weight)))
                .collect::<Vec<_>>(),
        }))
}

pub(crate) fn parse_unsorted_quadruples(
    mut edges: Vec<Quadruple>,
    verbose: bool,
) -> (usize, impl Iterator<Item = Result<Quadruple, String>>) {
    info!("Sorting edges.");
    edges.par_sort_by(|(src1, dst1, edt1, _), (src2, dst2, edt2, _)| {
        (*src1, *dst1, *edt1).cmp(&(*src2, *dst2, *edt2))
    });

    let edges_number = edges.len();
    let pb = get_loading_bar(verbose, "Building sorted graph", edges_number);

    (
        edges_number,
        edges.into_iter().progress_with(pb).map(Result::Ok),
    )
}

pub(crate) fn parse_integer_unsorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>>,
    directed: bool,
    directed_edge_list: bool,
    verbose: bool,
) -> Result<(usize, impl Iterator<Item = Result<Quadruple, String>> + 'a), String> {
    let edge_quadruples: Vec<Quadruple> = edges_iter
        .flat_map(|tuple| match tuple {
            Ok((src, dst, edt, weight)) => {
                if !directed && src != dst && !directed_edge_list {
                    vec![Ok((src, dst, edt, weight)), Ok((dst, src, edt, weight))]
                } else {
                    vec![Ok((src, dst, edt, weight))]
                }
            }
            Err(e) => vec![Err(e)],
        })
        .collect::<Result<Vec<Quadruple>, String>>()?;

    Ok(parse_unsorted_quadruples(edge_quadruples, verbose))
}

pub(crate) fn parse_string_unsorted_edges<'a>(
    // This parameter does not NEED a lifetime because it does NOT survive the function call
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    mut nodes: Vocabulary<NodeT>,
    directed: bool,
    directed_edge_list: bool,
    edge_list_is_correct: bool,
    has_edge_types: bool,
    verbose: bool,
    numeric_edge_type_ids: bool,
) -> Result<
    (
        usize,
        impl Iterator<Item = Result<Quadruple, String>> + 'a,
        Vocabulary<NodeT>,
        Option<Vocabulary<EdgeTypeT>>,
    ),
    String,
> {
    let mut edge_types_vocabulary = if has_edge_types {
        Some(Vocabulary::default().set_numeric_ids(numeric_edge_type_ids))
    } else {
        None
    };
    let (edges_number, edges_iter) = {
        let edges_iter = parse_edges_node_ids(edges_iter, edge_list_is_correct, &mut nodes);
        let edges_iter: Box<dyn Iterator<Item = Result<Quadruple, String>>> =
            if let Some(ets) = &mut edge_types_vocabulary {
                Box::new(parse_edge_type_ids_vocabulary(edges_iter, ets))
            } else {
                Box::new(edges_iter.map_ok(|(src, dst, _, weight)| (src, dst, None, weight)))
            };
        let edge_quadruples: Vec<Quadruple> = edges_iter
            .flat_map(|tuple| match tuple {
                Ok((src, dst, edt, weight)) => {
                    if !directed && src != dst && !directed_edge_list {
                        vec![Ok((src, dst, edt, weight)), Ok((dst, src, edt, weight))]
                    } else {
                        vec![Ok((src, dst, edt, weight))]
                    }
                }
                Err(e) => vec![Err(e)],
            })
            .collect::<Result<Vec<Quadruple>, String>>()?;

        parse_unsorted_quadruples(edge_quadruples, verbose)
    };
    info!("Building nodes reverse mapping.");
    nodes.build_reverse_mapping()?;
    if let Some(ets) = &mut edge_types_vocabulary {
        info!("Building edge types reverse mapping.");
        ets.build_reverse_mapping()?;
    }
    Ok((edges_number, edges_iter, nodes, edge_types_vocabulary))
}

pub(crate) fn build_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    ignore_duplicated_edges: bool,
    has_weights: bool,
    has_edge_types: bool,
    might_have_singletons: bool,
    might_have_singletons_with_selfloops: bool,
    might_have_trap_nodes: bool,
    directed: bool,
    edge_list_is_correct: bool,
) -> Result<
    (
        EliasFano,
        Option<EliasFano>,
        Option<Vec<Option<EdgeTypeT>>>,
        Option<Vec<WeightT>>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u8,
        u64,
        Option<BitVec<Lsb0, u8>>,
        Option<RoaringBitmap>,
    ),
    String,
> {
    info!("Started building of EliasFano edges data structure.");
    let node_bits = get_node_bits(nodes_number);
    let node_bit_mask = (1 << node_bits) - 1;
    let mut edges: EliasFano =
        EliasFano::new(encode_max_edge(nodes_number, node_bits), edges_number)?;

    // The graph might still contain duplicated edges, therefore the provided edges
    // number is a maximal value.
    let mut edge_type_ids: Option<Vec<Option<EdgeTypeT>>> = if has_edge_types {
        Some(Vec::with_capacity(edges_number))
    } else {
        None
    };

    let mut weights: Option<Vec<WeightT>> = if has_weights {
        Some(Vec::with_capacity(edges_number))
    } else {
        None
    };

    // The unique sources variable is equal to the set of nodes of the graph when
    // there are no singletons and the graph is undirected. Otherwise, if there is
    // a singleton node, that must not appear in this set.
    // We will use this set during the random walks and other graph algorithms
    // in order to obtain the nth source node. For this reason we cannot
    // use a bitvec here, since we need to execute an unchecked select when the
    // object is not equal to the set of the nodes to remap the nth source node
    // to the nth unique source node, excluding the eventual; singleton nodes.
    // Similarly, in directed graphs, only a subset of the nodes might be a
    // source as there might be traps.
    // In the case of directed graphs, we have additionally the might have trap nodes
    // parameter which allows to specify whether the graph is known to contain
    // trap nodes. The parameter only makes sense in directed graphs.
    // Since we expect that the average use case (when we arew not dealing with pathological graphs)
    // the following set should be relatively dense, when we know that the set of unique
    // sources will be needed but it will be equal to the nodes with edges set, we compute it
    // afterwards. This is because it is known that an Elias Fano data structure
    // uses more than twice the memory required by a bitvec to memorize a set of
    // dense values.
    let mut unique_sources: Option<EliasFano> =
        if directed && (might_have_trap_nodes || might_have_singletons) {
            Some(EliasFano::new(nodes_number as u64, nodes_number as usize)?)
        } else {
            None
        };
    // When the graph is either undirected or directed without trap nodes, the unique sources set and the
    // nodes with edges set are equal one another.
    // We need to compute the following set when it is not trivial, that is when
    // either the graph is undirected and there are no singletons or alternatively
    // when the graph is directed and there are neither trap nodes nor singletons.
    // Additionally, since we need this support data structure when computing the
    // number of singletons with selfloops, we need to create it also when it has
    // been specified that there might be singletons with selfloops.
    let mut not_singleton_nodes: Option<_> =
        if might_have_singletons || might_have_singletons_with_selfloops {
            Some(bitvec![Lsb0, u8; 0; nodes_number as usize])
        } else {
            None
        };

    // Last source inserted
    let mut last_src: NodeT = 0;
    let mut last_dst: NodeT = 0;
    let mut last_edge_type: Option<EdgeTypeT> = None;
    let mut unique_edges_number: EdgeT = 0;
    let mut unique_self_loop_number: NodeT = 0;
    let mut self_loop_number: EdgeT = 0;
    let mut forward_undirected_edges_counter: EdgeT = 0;
    let mut backward_undirected_edges_counter: EdgeT = 0;
    let mut not_singleton_node_number: NodeT =
        if might_have_singletons || might_have_singletons_with_selfloops {
            0
        } else {
            nodes_number
        };
    // This bitvec should be really sparse ON SANE GRAPHS
    // so we use a roaring bitvec to save memory.
    let mut singleton_nodes_with_self_loops = if might_have_singletons_with_selfloops {
        Some(RoaringBitmap::new())
    } else {
        None
    };

    let mut first = true;
    for value in edges_iter {
        let (src, dst, edge_type, weight) = value?;
        let different_src = last_src != src || first;
        let different_dst = last_dst != dst || first;
        let self_loop = src == dst;
        let different_edge_type = last_edge_type != edge_type || first;
        if !(different_src || different_dst || different_edge_type) {
            if ignore_duplicated_edges {
                continue;
            } else {
                return Err("A duplicated edge was found while building the graph.".to_owned());
            }
        }

        if let Some(ets) = &mut edge_type_ids {
            ets.push(edge_type);
        }
        match (&mut weights, weight) {
            (Some(ws), Some(w)) => {
                validate_weight(w)?;
                ws.push(w);
                Ok(())
            }
            (None, Some(_)) => Err(concat!(
                "A non-None weight was provided but no weights are expected ",
                "because the has_weights flag has been set to false."
            )),
            (Some(_), None) => Err(concat!(
                "A None weight was found.\n",
                "This might mean you have either provided a None weight to the edge list or ",
                "you may have an empty weight in your edge list file.\n",
                "If you intend to load this edge list WITHOUT weights, do not provide the ",
                "edge weights colum or column number.\n",
                "If you intend to load this edge with its weight, add a default weight."
            )),
            _ => Ok(()),
        }?;

        if !directed && !edge_list_is_correct {
            match src.cmp(&dst) {
                Ordering::Greater => {
                    // We retrieve the edge id of the forward edge, the one going from
                    // dst to src.
                    let maybe_edge_id = edges.rank(encode_edge(dst, src, node_bits));
                    // Now we need to find, starting from edge id, if the edge types are given,
                    // the correct edge id: if we are in a multi-graph the edge may be the same
                    // but have multiple edge types and hence be reported multiple times.
                    let maybe_edge_id = maybe_edge_id.and_then(|min_edge_id| {
                        edge_type_ids.as_ref().map_or(Some(min_edge_id), |ets| {
                            (min_edge_id
                                ..edges.unchecked_rank(encode_edge(dst, src + 1, node_bits)))
                                .find(|edge_id| ets[*edge_id as usize] == edge_type)
                        })
                    });
                    // Finally now we need to check if the weights of the two edges, if given
                    // are actually equal.
                    let has_unbalanced_undirected_edge = maybe_edge_id.map_or(true, |edge_id| {
                        weights.as_ref().map_or(false, |ws| {
                            (ws[edge_id as usize] - weight.unwrap()).abs() >= f32::EPSILON
                        })
                    });
                    if has_unbalanced_undirected_edge {
                        return Err(concat!(
                            "You are trying to load an undirected ",
                            "graph using the directed edge list ",
                            "paremeter that requires for ALL edges to ",
                            "be fully defined in both directions.\n",
                            "The edge list you have provided does not ",
                            "provide the edges in both directions.",
                        )
                        .to_string());
                    }
                    backward_undirected_edges_counter += 1
                }
                Ordering::Less => forward_undirected_edges_counter += 1,
                Ordering::Equal => {}
            }
        }
        last_edge_type = edge_type;
        edges.unchecked_push(encode_edge(src, dst, node_bits));
        if self_loop {
            self_loop_number += 1;
        }
        if different_src || different_dst {
            if let Some(nwe) = &mut not_singleton_nodes {
                for node in &[src, dst] {
                    unsafe {
                        let mut ptr = nwe.get_unchecked_mut(*node as usize);
                        if !*ptr {
                            *ptr = true;
                            if !self_loop || singleton_nodes_with_self_loops.is_none() {
                                not_singleton_node_number += 1;
                            } else {
                                if let Some(bitmap) = &mut singleton_nodes_with_self_loops {
                                    bitmap.insert(*node);
                                }
                                break;
                            }
                        } else if !self_loop
                            && singleton_nodes_with_self_loops
                                .as_mut()
                                .map_or(false, |bitmap| bitmap.remove(*node))
                        {
                            not_singleton_node_number += 1;
                        }
                    }
                }
            }
            unique_edges_number += 1;
            if self_loop {
                unique_self_loop_number += 1;
            }
            if different_src {
                if let Some(us) = &mut unique_sources {
                    us.unchecked_push(src as u64);
                }
            }
        }
        last_src = src;
        last_dst = dst;
        first = false;
    }

    if forward_undirected_edges_counter != backward_undirected_edges_counter {
        return Err(concat!(
            "You are trying to load an undirected graph ",
            "from a directed edge list but the edge list is not ",
            "complete."
        )
        .to_owned());
    }

    if let Some(ws) = &weights {
        if edges.len() != ws.len() {
            panic!(
                "The number of weights {} does not match the number of edges {}.",
                ws.len(),
                edges.len()
            );
        }
        if ws.is_empty() {
            weights = None;
        }
    }

    if let Some(ets) = &edge_type_ids {
        if edges.len() != ets.len() {
            panic!(
                "The number of edge types {} does not match the number of edges {}.",
                ets.len(),
                edges.len()
            );
        }

        if ets.is_empty() {
            edge_type_ids = None;
        }
    }

    if not_singleton_node_number > nodes_number {
        panic!(
            "There is an error in the constructor, the not singleton  node number '{}' is bigger than node number '{}'",
            not_singleton_node_number, nodes_number
        );
    }

    let singleton_nodes_with_self_loops_number = singleton_nodes_with_self_loops
        .as_ref()
        .map_or(0, |bitmap| bitmap.len() as NodeT);

    // While on internal methods nodes_number is always exact, the user may
    // provide a wrong value for nodes_number when loading a sorted csv.
    // If this happens, it might cause a slow down in the walk and other
    // currently unforseen consequences.
    if nodes_number == not_singleton_node_number + singleton_nodes_with_self_loops_number {
        unique_sources = None;
    }

    // When we have computed the nodes with edges set but we have left None
    // the unique sources elias fano, this is done to avoid using extra memory
    // for no reason. We need to create the elias fano object starting from the
    // nodes with edges now to normalize the returned values.
    if might_have_singletons
        && unique_sources.is_none()
        && nodes_number != not_singleton_node_number + singleton_nodes_with_self_loops_number
    {
        unique_sources = not_singleton_nodes
            .as_ref()
            .map_or(Ok::<_, String>(None), |nsns| {
                Ok(Some(EliasFano::from_iter(
                    nsns.iter_ones().into_iter().map(|x| x as u64),
                    nodes_number as u64,
                    not_singleton_node_number as usize
                        + singleton_nodes_with_self_loops_number as usize,
                )?))
            })?;
    }

    if !directed
        && unique_sources
            .as_ref()
            .map_or(false, |x| not_singleton_node_number > x.len() as NodeT)
    {
        panic!(
            "There is an error in the constructor, the not singleton node number '{}' is bigger than the len of unique sources which is '{}'",
            not_singleton_node_number, unique_sources.unwrap().len()
        );
    }

    Ok((
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_node_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
        not_singleton_nodes,
        singleton_nodes_with_self_loops,
    ))
}

fn parse_nodes(
    nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
    ignore_duplicated_nodes: bool,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_types_ids: bool,
    numeric_edge_node_ids: bool,
    has_node_types: bool,
) -> Result<(Vocabulary<NodeT>, Option<NodeTypeVocabulary>), String> {
    let mut nodes = Vocabulary::default()
        .set_numeric_ids(numeric_node_ids || numeric_edge_node_ids && nodes_iterator.is_none());

    let node_types = if let Some(ni) = nodes_iterator {
        // TODO: the following can likely be dealt with in a better way.
        let node_iterator = parse_node_ids(
            ni,
            ignore_duplicated_nodes,
            node_list_is_correct,
            &mut nodes,
        );
        // In the case there is a node types we need to add its proper iterator.
        if has_node_types {
            let mut node_types =
                NodeTypeVocabulary::default().set_numeric_ids(numeric_node_types_ids);
            for row in parse_node_type_ids(node_iterator, &mut node_types) {
                row?;
            }
            node_types.build_reverse_mapping()?;
            node_types.build_counts();

            if node_types.is_empty() {
                Ok(None)
            } else {
                Ok::<_, String>(Some(node_types))
            }
        } else {
            for row in node_iterator {
                row?;
            }
            Ok::<_, String>(None)
        }?
    } else {
        None
    };

    Ok((nodes, node_types))
}

pub(crate) fn parse_string_edges(
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    directed: bool,
    mut nodes: Vocabulary<NodeT>,
    numeric_edge_type_ids: bool,
    directed_edge_list: bool,
    edge_list_is_correct: bool,
    ignore_duplicated_edges: bool,
    has_edge_types: bool,
    has_weights: bool,
    might_have_singletons: bool,
    might_have_singletons_with_selfloops: bool,
    might_have_trap_nodes: bool,
) -> ParsedStringEdgesType {
    let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> =
        Vocabulary::default().set_numeric_ids(numeric_edge_type_ids);

    // This is not equivalent to nodes_iterator.is_some() because the iterator
    // could also be empty, this is a corner-case that might happen when over-filtering
    // or fuzzing or loading an empty file with improper configurations.
    // There might be singletons if the user has told us that there might be singletons
    // and the node list is not empty. If the node list is empty, then it is not possible
    // to have singletons.
    let might_have_singletons = !nodes.is_empty() && might_have_singletons;
    // If the graph is undirected there cannot be trap nodes
    let might_have_trap_nodes = directed && might_have_trap_nodes;

    let edges_iter = parse_sorted_edges(
        parse_edge_type_ids_vocabulary(
            parse_edges_node_ids(edges_iter, edge_list_is_correct, &mut nodes),
            &mut edge_types_vocabulary,
        ),
        directed,
        directed_edge_list,
    );

    let (
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
        not_singleton_nodes,
        singleton_nodes_with_self_loops,
    ) = build_edges(
        edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
        has_weights,
        has_edge_types,
        might_have_singletons,
        might_have_singletons_with_selfloops,
        might_have_trap_nodes,
        directed,
        edge_list_is_correct,
    )?;

    nodes.build_reverse_mapping()?;
    edge_types_vocabulary.build_reverse_mapping()?;
    let edge_types =
        EdgeTypeVocabulary::from_option_structs(edge_type_ids, optionify!(edge_types_vocabulary));

    Ok((
        edges,
        unique_sources,
        nodes,
        edge_types,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bit_mask,
        node_bits,
        not_singleton_nodes,
        singleton_nodes_with_self_loops,
    ))
}

pub(crate) fn parse_integer_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    ignore_duplicated_edges: bool,
    directed: bool,
    edge_list_is_correct: bool,
    has_edge_types: bool,
    has_weights: bool,
    might_have_singletons: bool,
    might_have_singletons_with_selfloops: bool,
    might_have_trap_nodes: bool,
) -> Result<
    (
        EliasFano,
        Option<EliasFano>,
        Option<EdgeTypeVocabulary>,
        Option<Vec<WeightT>>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u64,
        u8,
        Option<BitVec<Lsb0, u8>>,
        Option<RoaringBitmap>,
    ),
    String,
> {
    let (
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
        not_singleton_nodes,
        singleton_nodes_with_self_loops,
    ) = build_edges(
        edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
        has_weights,
        has_edge_types,
        might_have_singletons,
        might_have_singletons_with_selfloops,
        might_have_trap_nodes,
        directed,
        edge_list_is_correct,
    )?;

    let edge_types = EdgeTypeVocabulary::from_option_structs(edge_type_ids, edge_types_vocabulary);

    Ok((
        edges,
        unique_sources,
        edge_types,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bit_mask,
        node_bits,
        not_singleton_nodes,
        singleton_nodes_with_self_loops,
    ))
}

/// # Graph Constructors
impl Graph {
    pub(crate) fn build_graph<S: Into<String>>(
        edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
        edges_number: usize,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        edge_list_is_correct: bool,
        name: S,
        ignore_duplicated_edges: bool,
        has_edge_types: bool,
        has_weights: bool,
        might_have_singletons: bool,
        might_have_singletons_with_selfloops: bool,
        might_have_trap_nodes: bool,
    ) -> Result<Graph, String> {
        let (
            edges,
            unique_sources,
            edge_types,
            weights,
            unique_edges_number,
            self_loop_number,
            unique_self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            node_bit_mask,
            node_bits,
            not_singleton_nodes,
            singleton_nodes_with_self_loops,
        ) = parse_integer_edges(
            edges_iter,
            edges_number,
            nodes.len() as NodeT,
            edge_types_vocabulary,
            ignore_duplicated_edges,
            directed,
            edge_list_is_correct,
            has_edge_types,
            has_weights,
            might_have_singletons,
            might_have_singletons_with_selfloops,
            might_have_trap_nodes,
        )?;

        Ok(Graph::new(
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            weights,
            node_types,
            not_singleton_nodes,
            singleton_nodes_with_self_loops,
        ))
    }

    /// Create new Graph object from unsorted source.
    ///
    /// # Arguments
    ///
    /// TODO: UPDATE THE DOCSTRING!
    ///
    /// * edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
    ///     Iterator of the edges.
    /// * nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
    ///     Iterator of the nodes.
    /// * directed: bool,
    ///     Wether the graph should be directed or undirected.
    /// * ignore_duplicated_nodes: bool,
    ///     Wether to ignore duplicated nodes or to raise a proper exception.
    /// * ignore_duplicated_edges: bool,
    ///     Wether to ignore duplicated edges or to raise a proper exception.
    /// * skip_self_loops: bool,
    ///     Wether to skip self loops while reading the the edges iterator.
    pub fn from_string_unsorted<S: Into<String>>(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
        ignore_duplicated_nodes: bool,
        node_list_is_correct: bool,
        ignore_duplicated_edges: bool,
        edge_list_is_correct: bool,
        verbose: bool,
        numeric_edge_type_ids: bool,
        numeric_node_ids: bool,
        numeric_edge_node_ids: bool,
        numeric_node_types_ids: bool,
        has_node_types: bool,
        has_edge_types: bool,
        has_weights: bool,
        might_have_singletons: bool,
        might_have_singletons_with_selfloops: bool,
        might_have_trap_nodes: bool,
    ) -> Result<Graph, String> {
        check_numeric_ids_compatibility(
            nodes_iterator.is_some(),
            numeric_node_ids,
            numeric_edge_node_ids,
        )?;

        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            node_list_is_correct,
            numeric_node_ids,
            numeric_node_types_ids,
            numeric_edge_node_ids,
            has_node_types,
        )?;

        // This is not equivalent to nodes_iterator.is_some() because the iterator
        // could also be empty, this is a corner-case that might happen when over-filtering
        // or fuzzing or loading an empty file with improper configurations.
        // There might be singletons if the user has told us that there might be singletons
        // and the node list is not empty. If the node list is empty, then it is not possible
        // to have singletons.
        let might_have_singletons = !nodes.is_empty() && might_have_singletons;
        // If the graph is undirected there cannot be trap nodes
        let might_have_trap_nodes = directed && might_have_trap_nodes;

        info!("Parse unsorted edges.");
        let (edges_number, edges_iterator, nodes, edge_types_vocabulary) =
            parse_string_unsorted_edges(
                edges_iterator,
                nodes,
                directed,
                directed_edge_list,
                edge_list_is_correct,
                has_edge_types,
                verbose,
                numeric_edge_type_ids,
            )?;

        Graph::build_graph(
            edges_iterator,
            edges_number,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
            edge_list_is_correct || !directed_edge_list,
            name,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights,
            might_have_singletons,
            might_have_singletons_with_selfloops,
            might_have_trap_nodes,
        )
    }

    /// Create new Graph object from unsorted source.
    ///
    /// # Arguments
    ///
    /// * edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
    ///     Iterator of the edges.
    /// * nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
    ///     Iterator of the nodes.
    /// * directed: bool,
    ///     Wether the graph should be directed or undirected.
    /// * ignore_duplicated_nodes: bool,
    ///     Wether to ignore duplicated nodes or to raise a proper exception.
    /// * ignore_duplicated_edges: bool,
    ///     Wether to ignore duplicated edges or to raise a proper exception.
    /// * skip_self_loops: bool,
    ///     Wether to skip self loops while reading the the edges iterator.
    pub fn from_integer_unsorted(
        edges_iterator: impl Iterator<
            Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>,
        >,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        name: String,
        ignore_duplicated_edges: bool,
        has_edge_types: bool,
        has_weights: bool,
        verbose: bool,
        might_have_singletons: bool,
        might_have_singletons_with_selfloops: bool,
        might_have_trap_nodes: bool,
    ) -> Result<Graph, String> {
        let (edges_number, edges_iterator) =
            parse_integer_unsorted_edges(edges_iterator, directed, true, verbose)?;

        Graph::build_graph(
            edges_iterator,
            edges_number,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
            true,
            name,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights,
            might_have_singletons,
            might_have_singletons_with_selfloops,
            might_have_trap_nodes,
        )
    }

    /// Create new Graph object from sorted sources.
    pub fn from_string_sorted<S: Into<String>>(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
        directed: bool,
        directed_edge_list: bool,
        ignore_duplicated_nodes: bool,
        node_list_is_correct: bool,
        ignore_duplicated_edges: bool,
        edge_list_is_correct: bool,
        edges_number: usize,
        mut nodes_number: NodeT,
        numeric_edge_type_ids: bool,
        numeric_node_ids: bool,
        numeric_edge_node_ids: bool,
        numeric_node_types_ids: bool,
        has_node_types: bool,
        has_edge_types: bool,
        has_weights: bool,
        might_have_singletons: bool,
        might_have_singletons_with_selfloops: bool,
        might_have_trap_nodes: bool,
        name: S,
    ) -> Result<Graph, String> {
        check_numeric_ids_compatibility(
            nodes_iterator.is_some(),
            numeric_node_ids,
            numeric_edge_node_ids,
        )?;
        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            node_list_is_correct,
            numeric_node_ids,
            numeric_node_types_ids,
            numeric_edge_node_ids,
            has_node_types,
        )?;

        if !nodes.is_empty() {
            nodes_number = nodes.len() as NodeT;
        }

        let (
            edges,
            unique_sources,
            nodes,
            edge_types,
            weights,
            unique_edges_number,
            self_loop_number,
            unique_self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            node_bit_mask,
            node_bits,
            not_singleton_nodes,
            singleton_nodes_with_self_loops,
        ) = parse_string_edges(
            edges_iterator,
            edges_number,
            nodes_number,
            directed,
            nodes,
            numeric_edge_type_ids,
            directed_edge_list,
            edge_list_is_correct,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights,
            might_have_singletons,
            might_have_singletons_with_selfloops,
            might_have_trap_nodes,
        )?;

        Ok(Graph::new(
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            weights,
            node_types,
            not_singleton_nodes,
            singleton_nodes_with_self_loops,
        ))
    }
}

use super::*;
use counter::Counter;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use roaring::{RoaringBitmap, RoaringTreemap};
use std::collections::HashSet;
use std::iter::FromIterator;
use vec_rand::xorshift::xorshift as rand_u64;
use vec_rand::{gen_random_vec, sample_uniform};

/// # Holdouts.
impl Graph {
    /// Returns Graph with given amount of negative edges as positive edges.
    ///
    /// The graph generated may be used as a testing negatives partition to be
    /// fed into the argument "graph_to_avoid" of the link_prediction or the
    /// skipgrams algorithm.
    ///
    ///
    /// # Arguments
    ///
    /// * `random_state`: EdgeT - random_state to use to reproduce negative edge set.
    /// * `negatives_number`: EdgeT - Number of negatives edges to include.
    /// * `seed_graph`: Option<Graph> - Optional graph to use to filter the negative edges. The negative edges generated when this variable is provided will always have a node within this graph.
    /// * `only_from_same_component`: bool - whether to sample negative edges only from nodes that are from the same component.
    /// * `verbose`: bool - whether to show the loading bar.
    ///
    pub fn sample_negatives(
        &self,
        mut random_state: EdgeT,
        negatives_number: EdgeT,
        seed_graph: Option<&Graph>,
        only_from_same_component: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        if negatives_number == 0 {
            return Err(String::from("The number of negatives cannot be zero."));
        }
        let seed_nodes: Option<RoaringBitmap> = if let Some(sg) = &seed_graph {
            if !self.overlaps(&sg)? {
                return Err(String::from(
                    "The given seed graph does not overlap with the current graph instance.",
                ));
            }
            Some(
                sg.iter_nodes()
                    .map(|(_, node_name, _, _)| self.get_unchecked_node_id_by_node_name(&node_name))
                    .collect::<RoaringBitmap>(),
            )
        } else {
            None
        };
        // In a complete directed graph allowing selfloops with N nodes there are N^2
        // edges. In a complete directed graph without selfloops there are N*(N-1) edges.
        // We can rewrite the first formula as (N*(N-1)) + N.
        //
        // In a complete undirected graph allowing selfloops with N nodes there are
        // (N*(N-1))/2 + N edges.

        // Here we use unique edges number because on a multigraph the negative
        // edges cannot have an edge type.
        let nodes_number = self.get_nodes_number() as EdgeT;

        // whether to sample negative edges only from the same connected component.
        let (node_components, mut complete_edges_number) = if only_from_same_component {
            let node_components = self.get_node_components_vector(verbose);
            let complete_edges_number: EdgeT = Counter::init(node_components.clone())
                .into_iter()
                .map(|(_, nodes_number): (_, &usize)| {
                    let mut edge_number = (*nodes_number * (*nodes_number - 1)) as EdgeT;
                    if !self.is_directed() {
                        edge_number /= 2;
                    }
                    edge_number
                })
                .sum();
            (Some(node_components), complete_edges_number)
        } else {
            let mut edge_number = nodes_number * (nodes_number - 1);
            if !self.is_directed() {
                edge_number /= 2;
            }
            (None, edge_number)
        };

        // Here we compute the number of edges that a complete graph would have if it had the same number of nodes
        // of the current graph. Moreover, the complete graph will have selfloops IFF the current graph has at
        // least one of them.
        if self.has_selfloops() {
            complete_edges_number += nodes_number;
        }

        // Now we compute the maximum number of negative edges that we can actually generate
        let max_negative_edges = complete_edges_number - self.get_unique_edges_number();

        // We check that the number of requested negative edges is compatible with the
        // current graph instance.
        if negatives_number > max_negative_edges {
            return Err(format!(
                concat!(
                    "The requested negatives number {} is more than the ",
                    "number of negative edges that exist in the graph ({})."
                ),
                negatives_number, max_negative_edges
            ));
        }

        // As the above check, it is not possible to generate some negative
        // graphs when some conditions.
        if negatives_number % 2 == 1 && !self.is_directed() && !self.has_selfloops() {
            return Err(format!(
                concat!(
                    "The requested negatives number {} is an odd number and ",
                    "the graph is neither directed nor has selfloops, so it is ",
                    "not possible to sample an odd number of edges."
                ),
                negatives_number
            ));
        }

        let pb1 = get_loading_bar(
            verbose,
            "Computing negative edges",
            negatives_number as usize,
        );

        // xorshift breaks if the random_state is zero
        // so we initialize xor it with a constat
        // to mitigate this problem
        random_state ^= SEED_XOR as EdgeT;

        let mut negative_edges_hashset = HashSet::with_capacity(negatives_number as usize);
        let mut last_length = 0;
        let mut sampling_round: usize = 0;

        // randomly extract negative edges until we have the choosen number
        while negative_edges_hashset.len() < negatives_number as usize {
            // generate two random_states for reproducibility porpouses
            let src_random_state = rand_u64(random_state);
            let dst_random_state = rand_u64(src_random_state);
            random_state = rand_u64(dst_random_state);

            let tmp_tb = get_loading_bar(
                verbose,
                format!("Negatives sampling round {}", sampling_round).as_ref(),
                negatives_number as usize,
            );
            sampling_round += 1;

            // generate the random edge-sources
            let sampled_edge_ids = gen_random_vec(negatives_number as usize, src_random_state)
                .into_par_iter()
                .zip(gen_random_vec(negatives_number as usize, dst_random_state).into_par_iter())
                // convert them to plain (src, dst)
                .progress_with(tmp_tb)
                .filter_map(|(src_seed, dst_seed)| {
                    let src = sample_uniform(nodes_number as u64, src_seed as u64) as NodeT;
                    let dst = sample_uniform(nodes_number as u64, dst_seed as u64) as NodeT;
                    if !self.is_directed() && src > dst {
                        return None;
                    }

                    if !self.has_selfloops() && src == dst {
                        return None;
                    }

                    if let Some(sn) = &seed_nodes {
                        if !sn.contains(src) && !sn.contains(dst) {
                            return None;
                        }
                    }
                    if let Some(ncs) = &node_components {
                        if ncs[src as usize] != ncs[dst as usize] {
                            return None;
                        }
                    }
                    // If the edge is not a self-loop or the user allows self-loops and
                    // the graph is directed or the edges are inserted in a way to avoid
                    // inserting bidirectional edges.
                    match self.has_edge_by_node_ids(src, dst) {
                        true => None,
                        false => Some(self.encode_edge(src, dst)),
                    }
                })
                .collect::<Vec<EdgeT>>();

            let pb3 = get_loading_bar(
                verbose,
                format!(
                    "Inserting negative graph edges (iteration {})",
                    sampling_round
                )
                .as_ref(),
                negatives_number as usize,
            );

            for edge_id in sampled_edge_ids.iter().progress_with(pb3) {
                if negative_edges_hashset.len() >= negatives_number as usize {
                    break;
                }
                negative_edges_hashset.insert(*edge_id);
            }

            if sampling_round > 50000 {
                panic!("Deadlock in sampling negatives!");
            }

            pb1.inc((negative_edges_hashset.len() - last_length as usize) as u64);
            last_length = negative_edges_hashset.len();
        }

        pb1.finish();

        Graph::from_integer_unsorted(
            negative_edges_hashset.into_iter().flat_map(|edge| {
                let (src, dst) = self.decode_edge(edge);
                if !self.is_directed() && src != dst {
                    vec![Ok((src, dst, None, None)), Ok((dst, src, None, None))]
                } else {
                    vec![Ok((src, dst, None, None))]
                }
            }),
            self.nodes.clone(),
            self.node_types.clone(),
            None,
            self.directed,
            format!("Negative {}", self.name.clone()),
            false,
            false,
            false,
            verbose,
            true,
            self.has_selfloops(),
            true,
        )
    }

    /// Compute the training and validation elements number from the training rate
    fn get_holdouts_elements_number(
        &self,
        train_size: f64,
        total_elements: usize,
    ) -> Result<(usize, usize), String> {
        if train_size <= 0.0 || train_size >= 1.0 {
            return Err(String::from("Train rate must be strictly between 0 and 1."));
        }
        if self.directed && self.get_directed_edges_number() == 1
            || !self.directed && self.get_directed_edges_number() == 2
        {
            return Err(String::from(
                "The current graph instance has only one edge. You cannot build an holdout with one edge.",
            ));
        }
        let train_elements_number = (total_elements as f64 * train_size) as usize;
        let valid_elements_number = total_elements - train_elements_number;

        if train_elements_number == 0 || train_elements_number >= total_elements {
            return Err(String::from(
                "The training set has 0 elements! Change the training rate.",
            ));
        }
        if valid_elements_number == 0 {
            return Err(String::from(
                "The validation set has 0 elements! Change the training rate.",
            ));
        }

        Ok((train_elements_number, valid_elements_number))
    }

    /// Compute the training and validation edges number from the training rate
    fn get_holdouts_edges_number(
        &self,
        train_size: f64,
        include_all_edge_types: bool,
    ) -> Result<(EdgeT, EdgeT), String> {
        if self.directed && self.get_directed_edges_number() == 1
            || !self.directed && self.get_directed_edges_number() == 2
        {
            return Err(String::from(
                "The current graph instance has only one edge. You cannot build an holdout with one edge.",
            ));
        }
        let total_edges_number = if include_all_edge_types {
            self.unique_edges_number
        } else {
            self.get_directed_edges_number()
        };

        let (train_edges, test_edges) =
            self.get_holdouts_elements_number(train_size, total_edges_number as usize)?;
        Ok((train_edges as EdgeT, test_edges as EdgeT))
    }

    fn edge_holdout(
        &self,
        random_state: EdgeT,
        valid_edges_number: EdgeT,
        include_all_edge_types: bool,
        user_condition: impl Fn(EdgeT, NodeT, NodeT, Option<EdgeTypeT>) -> bool,
        verbose: bool,
        train_graph_might_have_singletons: bool,
        train_graph_might_have_singletons_with_selfloops: bool,
    ) -> Result<(Graph, Graph), String> {
        let pb1 = get_loading_bar(
            verbose,
            "Picking validation edges",
            valid_edges_number as usize,
        );

        // generate and shuffle the indices of the edges
        let mut rng = SmallRng::seed_from_u64(random_state ^ SEED_XOR as EdgeT);
        let mut edge_indices: Vec<EdgeT> = (0..self.get_directed_edges_number()).collect();
        edge_indices.shuffle(&mut rng);

        let mut valid_edges_bitmap = RoaringTreemap::new();
        let mut last_length = 0;

        for (edge_id, (src, dst, edge_type)) in edge_indices
            .into_iter()
            .map(|edge_id| (edge_id, self.get_edge_triple(edge_id)))
        {
            // If the graph is undirected and we have extracted an edge that is a
            // simmetric one, we can skip this iteration.
            if !self.directed && src > dst {
                continue;
            }

            // We stop adding edges when we have reached the minimum amount.
            if user_condition(edge_id, src, dst, edge_type) {
                // Compute the forward edge ids that are required.
                valid_edges_bitmap.extend(self.compute_edge_ids_vector(
                    edge_id,
                    src,
                    dst,
                    include_all_edge_types,
                ));

                // If the graph is undirected
                if !self.directed {
                    // we compute also the backward edge ids that are required.
                    valid_edges_bitmap.extend(self.compute_edge_ids_vector(
                        self.get_unchecked_edge_id_by_node_ids(dst, src, edge_type),
                        dst,
                        src,
                        include_all_edge_types,
                    ));
                }
                pb1.inc(valid_edges_bitmap.len() - last_length);
                last_length = valid_edges_bitmap.len();
            }

            // We stop the iteration when we found all the edges.
            if valid_edges_bitmap.len() >= valid_edges_number {
                break;
            }
        }

        if valid_edges_bitmap.len() < valid_edges_number {
            let actual_valid_edges_number = valid_edges_bitmap.len();
            return Err(format!(
                concat!(
                    "With the given configuration for the holdout, it is not possible to ",
                    "generate a validation set composed of {valid_edges_number} edges from the current graph.\n",
                    "The validation set can be composed of at most {actual_valid_edges_number} edges.\n"
                ),
                valid_edges_number=valid_edges_number,
                actual_valid_edges_number=actual_valid_edges_number,
            ));
        }

        // Creating the loading bar for the building of both the training and validation.
        let pb_valid = get_loading_bar(
            verbose,
            "Building the valid partition",
            valid_edges_bitmap.len() as usize,
        );
        let pb_train = get_loading_bar(
            verbose,
            "Building the train partition",
            (self.get_directed_edges_number() - valid_edges_bitmap.len()) as usize,
        );

        Ok((
            Graph::build_graph(
                (0..self.get_directed_edges_number())
                    .filter(|edge_id| !valid_edges_bitmap.contains(*edge_id))
                    .progress_with(pb_train)
                    .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
                self.get_directed_edges_number() as usize - valid_edges_bitmap.len() as usize,
                self.nodes.clone(),
                self.node_types.clone(),
                self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
                self.directed,
                true,
                format!("{} training", self.name.clone()),
                true,
                self.has_edge_types(),
                self.has_weights(),
                train_graph_might_have_singletons,
                train_graph_might_have_singletons_with_selfloops,
                true,
            )?,
            Graph::build_graph(
                valid_edges_bitmap
                    .iter()
                    .progress_with(pb_valid)
                    .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
                valid_edges_bitmap.len() as usize,
                self.nodes.clone(),
                self.node_types.clone(),
                self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
                self.directed,
                true,
                format!("{} testing", self.name.clone()),
                true,
                self.has_edge_types(),
                self.has_weights(),
                true,
                self.has_selfloops(),
                true,
            )?,
        ))
    }

    /// Returns holdout for training ML algorithms on the graph structure.
    ///
    /// The holdouts returned are a tuple of graphs. The first one, which
    /// is the training graph, is garanteed to have the same number of
    /// graph components as the initial graph. The second graph is the graph
    /// meant for testing or validation of the algorithm, and has no garantee
    /// to be connected. It will have at most (1-train_size) edges,
    /// as the bound of connectivity which is required for the training graph
    /// may lead to more edges being left into the training partition.
    ///
    /// In the option where a list of edge types has been provided, these
    /// edge types will be those put into the validation set.
    ///
    /// # Arguments
    ///
    /// * `random_state`: NodeT - The random_state to use for the holdout,
    /// * `train_size`: f64 - Rate target to reserve for training.
    /// * `edge_types`: Option<Vec<String>> - Edge types to be selected for in the validation set.
    /// * `include_all_edge_types`: bool - whether to include all the edges between two nodes.
    /// * `verbose`: bool - whether to show the loading bar.
    ///
    ///
    pub fn connected_holdout(
        &self,
        random_state: EdgeT,
        train_size: f64,
        edge_types: Option<Vec<Option<String>>>,
        include_all_edge_types: bool,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        if train_size <= 0.0 || train_size >= 1.0 {
            return Err(String::from("Train rate must be strictly between 0 and 1."));
        }

        let edge_type_ids = edge_types.map_or(Ok::<_, String>(None), |ets| {
            Ok(Some(
                self.get_edge_type_ids_by_edge_type_names(ets)?
                    .into_iter()
                    .collect::<HashSet<Option<EdgeTypeT>>>(),
            ))
        })?;

        let tree = self
            .random_spanning_arborescence_kruskal(random_state, &edge_type_ids, verbose)
            .0;

        let edge_factor = if self.is_directed() { 1 } else { 2 };
        let train_edges_number = (self.get_directed_edges_number() as f64 * train_size) as usize;
        let mut valid_edges_number =
            (self.get_directed_edges_number() as f64 * (1.0 - train_size)) as EdgeT;

        if let Some(etis) = &edge_type_ids {
            let selected_edges_number: EdgeT = etis
                .iter()
                .map(|et| self.get_unchecked_edge_count_by_edge_type_id(*et) as EdgeT)
                .sum();
            valid_edges_number = (selected_edges_number as f64 * (1.0 - train_size)) as EdgeT;
        }

        if tree.len() * edge_factor > train_edges_number {
            return Err(format!(
                concat!(
                    "The given spanning tree of the graph contains {} edges ",
                    "that is more than the required training edges number {}.\n",
                    "This makes impossible to create a validation set using ",
                    "{} edges.\nIf possible, you should increase the ",
                    "train_size parameter which is currently equal to ",
                    "{}.\nThe deny map, by itself, is requiring at least ",
                    "a train rate of {}."
                ),
                tree.len() * edge_factor,
                train_edges_number,
                valid_edges_number,
                train_size,
                (tree.len() * edge_factor) as f64 / train_edges_number as f64
            ));
        }

        self.edge_holdout(
            random_state,
            valid_edges_number,
            include_all_edge_types,
            |_, src, dst, edge_type| {
                let is_in_tree = tree.contains(&(src, dst));
                let singleton_self_loop =
                    src == dst && self.get_node_degree_by_node_id(src).unwrap() == 1;
                let correct_edge_type = edge_type_ids
                    .as_ref()
                    .map_or(true, |etis| etis.contains(&edge_type));
                // The tree must not contain the provided edge ID
                // And this is not a self-loop edge with degree 1
                // And the edge type of the edge ID is within the provided edge type
                !is_in_tree && !singleton_self_loop && correct_edge_type
            },
            verbose,
            self.has_singletons(),
            self.has_singleton_nodes_with_self_loops(),
        )
    }

    /// Returns random holdout for training ML algorithms on the graph edges.
    ///
    /// The holdouts returned are a tuple of graphs. In neither holdouts the
    /// graph connectivity is necessarily preserved. To maintain that, use
    /// the method `connected_holdout`.
    ///
    /// # Arguments
    ///
    /// * `random_state`: NodeT - The random_state to use for the holdout,
    /// * `train_size`: f64 - rate target to reserve for training
    /// * `include_all_edge_types`: bool - whether to include all the edges between two nodes.
    /// * `edge_types`: Option<Vec<String>> - The edges to include in validation set.
    /// * `min_number_overlaps`: Option<usize> - The minimum number of overlaps to include the edge into the validation set.
    /// * `verbose`: bool - whether to show the loading bar.
    ///
    pub fn random_holdout(
        &self,
        random_state: EdgeT,
        train_size: f64,
        include_all_edge_types: bool,
        edge_types: Option<Vec<Option<String>>>,
        min_number_overlaps: Option<EdgeT>,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        let (_, valid_edges_number) =
            self.get_holdouts_edges_number(train_size, include_all_edge_types)?;
        let edge_type_ids = edge_types.map_or(Ok::<_, String>(None), |ets| {
            Ok(Some(
                self.get_edge_type_ids_by_edge_type_names(ets)?
                    .into_iter()
                    .collect::<HashSet<Option<EdgeTypeT>>>(),
            ))
        })?;
        if min_number_overlaps.is_some() && !self.is_multigraph() {
            return Err("Current graph is not a multigraph!".to_string());
        }
        self.edge_holdout(
            random_state,
            valid_edges_number,
            include_all_edge_types,
            |_, src, dst, edge_type| {
                // If a list of edge types was provided and the edge type
                // of the current edge is not within the provided list,
                // we skip the current edge.
                if !edge_type_ids
                    .as_ref()
                    .map_or(true, |etis| etis.contains(&edge_type))
                {
                    return false;
                }
                // If a minimum number of overlaps was provided and the current
                // edge has not the required minimum amount of overlaps.
                if let Some(mno) = min_number_overlaps {
                    if self.get_unchecked_edge_degreee_by_node_ids(src, dst) < mno {
                        return false;
                    }
                }
                // Otherwise we accept the provided edge for the validation set
                true
            },
            verbose,
            // Singletons may be generated during the holdouts process
            true,
            // Singletons with selfloops may be generated during the holdouts process only when there are selfloops in the graph
            self.has_selfloops(),
        )
    }

    /// Returns node-label holdout for training ML algorithms on the graph node labels.
    ///
    /// # Arguments
    ///
    /// * `train_size`: f64 - rate target to reserve for training,
    /// * `use_stratification`: bool - Whether to use node-label stratification,
    /// * `random_state`: NodeT - The random_state to use for the holdout,
    ///
    pub fn node_label_holdout(
        &self,
        train_size: f64,
        use_stratification: bool,
        random_state: EdgeT,
    ) -> Result<(Graph, Graph), String> {
        if !self.has_node_types() {
            return Err("The current graph does not have node types.".to_string());
        }
        if use_stratification {
            if self.has_multilabel_node_types() {
                return Err("It is impossible to create a stratified holdout when the graph has multi-label node types.".to_string());
            }
            if self.get_minimum_node_types_number() < 2 {
                return Err("It is impossible to create a stratified holdout when the graph has node types with cardinality one.".to_string());
            }
        }

        // Compute the vectors with the indices of the nodes which node type matches
        // therefore the expected shape is:
        // (node_types_number, number of nodes of that node type)
        let node_sets: Vec<Vec<NodeT>> = self
            .node_types
            .as_ref()
            .map(|nts| {
                if use_stratification {
                    // Initialize the vectors for each node type
                    let mut node_sets: Vec<Vec<NodeT>> =
                        vec![Vec::new(); self.get_node_types_number() as usize];
                    // itering over the indices and adding each node to the
                    // vector of the corresponding node type.
                    nts.ids.iter().enumerate().for_each(|(node_id, node_type)| {
                        // if the node has a node_type
                        if let Some(nt) = node_type {
                            // Get the index of the correct node type vector.
                            node_sets[nt[0] as usize].push(node_id as NodeT);
                        };
                    });

                    node_sets
                } else {
                    // just compute a vector with a single vector of the indices
                    //  of the nodes with node
                    vec![nts
                        .ids
                        .iter()
                        .enumerate()
                        .filter_map(|(node_id, node_type)| {
                            node_type.as_ref().map(|_| node_id as NodeT)
                        })
                        .collect()]
                }
            })
            .unwrap();

        // initialize the seed for a re-producible shuffle
        let mut rnd = SmallRng::seed_from_u64(random_state ^ SEED_XOR as u64);

        // Allocate the vectors for the nodes of each
        let mut train_node_types = vec![None; self.get_nodes_number() as usize];
        let mut test_node_types = vec![None; self.get_nodes_number() as usize];

        for mut node_set in node_sets {
            // Shuffle in a reproducible way the nodes of the current node_type
            node_set.shuffle(&mut rnd);
            // Compute how many of these nodes belongs to the training set
            let (train_size, _) = self.get_holdouts_elements_number(train_size, node_set.len())?;
            // add the nodes to the relative vectors
            node_set[..train_size].iter().for_each(|node_id| {
                train_node_types[*node_id as usize] =
                    self.get_unchecked_node_type_id_by_node_id(*node_id)
            });
            node_set[train_size..].iter().for_each(|node_id| {
                test_node_types[*node_id as usize] =
                    self.get_unchecked_node_type_id_by_node_id(*node_id)
            });
        }

        // Clone the current graph
        // here we could manually initialize the clones so that we don't waste
        // time and memory cloning the node_types which will be immediately
        // overwrite. We argue that this should not be impactfull so we prefer
        // to prioritze the simplicity of the code
        let mut train_graph = self.clone();
        let mut test_graph = self.clone();

        // Replace the node_types with the one computes above
        train_graph.node_types = NodeTypeVocabulary::from_structs(
            train_node_types,
            self.node_types.as_ref().map(|ntv| ntv.vocabulary.clone()),
        );
        test_graph.node_types = NodeTypeVocabulary::from_structs(
            test_node_types,
            self.node_types.as_ref().map(|ntv| ntv.vocabulary.clone()),
        );

        Ok((train_graph, test_graph))
    }

    /// Returns edge-label holdout for training ML algorithms on the graph edge labels.
    ///
    /// # Arguments
    ///
    /// * `train_size`: f64 - rate target to reserve for training,
    /// * `use_stratification`: bool - Whether to use edge-label stratification,
    /// * `random_state`: EdgeT - The random_state to use for the holdout,
    ///
    pub fn edge_label_holdout(
        &self,
        train_size: f64,
        use_stratification: bool,
        random_state: EdgeT,
    ) -> Result<(Graph, Graph), String> {
        if !self.has_edge_types() {
            return Err("The current graph does not have edge types.".to_string());
        }
        if use_stratification && self.get_minimum_edge_types_number() < 2 {
            return Err("It is impossible to create a stratified holdout when the graph has edge types with cardinality one.".to_string());
        }

        // Compute the vectors with the indices of the edges which edge type matches
        // therefore the expected shape is:
        // (edge_types_number, number of edges of that edge type)
        let edge_sets: Vec<Vec<EdgeT>> = self
            .edge_types
            .as_ref()
            .map(|nts| {
                if use_stratification {
                    // Initialize the vectors for each edge type
                    let mut edge_sets: Vec<Vec<EdgeT>> =
                        vec![Vec::new(); self.get_edge_types_number() as usize];
                    // itering over the indices and adding each edge to the
                    // vector of the corresponding edge type.
                    nts.ids.iter().enumerate().for_each(|(edge_id, edge_type)| {
                        // if the edge has a edge_type
                        if let Some(et) = edge_type {
                            // Get the index of the correct edge type vector.
                            edge_sets[*et as usize].push(edge_id as EdgeT);
                        };
                    });

                    edge_sets
                } else {
                    // just compute a vector with a single vector of the indices
                    //  of the edges with edge
                    vec![nts
                        .ids
                        .iter()
                        .enumerate()
                        .filter_map(|(edge_id, edge_type)| {
                            edge_type.as_ref().map(|_| edge_id as EdgeT)
                        })
                        .collect()]
                }
            })
            .unwrap();

        // initialize the seed for a re-producible shuffle
        let mut rnd = SmallRng::seed_from_u64(random_state ^ SEED_XOR as u64);

        // Allocate the vectors for the edges of each
        let mut train_edge_types = vec![None; self.get_directed_edges_number() as usize];
        let mut test_edge_types = vec![None; self.get_directed_edges_number() as usize];

        for mut edge_set in edge_sets {
            // Shuffle in a reproducible way the edges of the current edge_type
            edge_set.shuffle(&mut rnd);
            // Compute how many of these edges belongs to the training set
            let (train_size, _) = self.get_holdouts_elements_number(train_size, edge_set.len())?;
            // add the edges to the relative vectors
            edge_set[..train_size].iter().for_each(|edge_id| {
                train_edge_types[*edge_id as usize] =
                    self.get_unchecked_edge_type_by_edge_id(*edge_id)
            });
            edge_set[train_size..].iter().for_each(|edge_id| {
                test_edge_types[*edge_id as usize] =
                    self.get_unchecked_edge_type_by_edge_id(*edge_id)
            });
        }

        // Clone the current graph
        // here we could manually initialize the clones so that we don't waste
        // time and memory cloning the edge_types which will be immediately
        // overwrite. We argue that this should not be impactfull so we prefer
        // to prioritze the simplicity of the code
        let mut train_graph = self.clone();
        let mut test_graph = self.clone();

        // Replace the edge_types with the one computes above
        train_graph.edge_types = Some(EdgeTypeVocabulary::from_structs(
            train_edge_types,
            self.edge_types
                .as_ref()
                .map(|etv| etv.vocabulary.clone())
                .unwrap(),
        ));
        test_graph.edge_types = Some(EdgeTypeVocabulary::from_structs(
            test_edge_types,
            self.edge_types
                .as_ref()
                .map(|etv| etv.vocabulary.clone())
                .unwrap(),
        ));

        Ok((train_graph, test_graph))
    }

    /// Returns subgraph with given number of nodes.
    ///
    /// This method creates a subset of the graph starting from a random node
    /// sampled using given random_state and includes all neighbouring nodes until
    /// the required number of nodes is reached. All the edges connecting any
    /// of the selected nodes are then inserted into this graph.
    ///
    /// This is meant to execute distributed node embeddings.
    /// It may also sample singleton nodes.
    ///
    /// # Arguments
    ///
    /// * `random_state`: usize - Random random_state to use.
    /// * `nodes_number`: usize - Number of nodes to extract.
    /// * `verbose`: bool - whether to show the loading bar.
    ///
    pub fn random_subgraph(
        &self,
        random_state: usize,
        nodes_number: NodeT,
        verbose: bool,
    ) -> Result<Graph, String> {
        if nodes_number <= 1 {
            return Err(String::from("Required nodes number must be more than 1."));
        }
        let not_singleton_nodes_number = self.get_not_singleton_nodes_number();
        if nodes_number > not_singleton_nodes_number {
            return Err(format!(
                concat!(
                    "Required number of nodes ({}) is more than available ",
                    "number of nodes ({}) that have edges in current graph."
                ),
                nodes_number, not_singleton_nodes_number
            ));
        }

        // Creating the loading bars
        let pb1 = get_loading_bar(verbose, "Sampling nodes subset", nodes_number as usize);
        let pb2 = get_loading_bar(verbose, "Computing subgraph edges", nodes_number as usize);
        let pb3 = get_loading_bar(
            verbose,
            "Building subgraph",
            self.get_directed_edges_number() as usize,
        );

        // Creating the random number generator
        let mut rnd = SmallRng::seed_from_u64((random_state ^ SEED_XOR) as u64);

        // Nodes indices
        let mut nodes: Vec<NodeT> = (0..self.get_nodes_number()).collect();

        // Shuffling the components using the given random_state.
        nodes.shuffle(&mut rnd);

        // Initializing stack and set of nodes
        let mut unique_nodes = RoaringBitmap::new();
        let mut stack: Vec<NodeT> = Vec::new();

        // We iterate on the components
        'outer: for node in nodes.iter() {
            // If the current node is a trap there is no need to continue with the current loop.
            if self.is_node_trap_by_node_id(*node).unwrap() {
                continue;
            }
            stack.push(*node);
            while !stack.is_empty() {
                let src = stack.pop().unwrap();
                for dst in self.iter_node_neighbours_ids(src) {
                    if !unique_nodes.contains(dst) && src != dst {
                        stack.push(dst);
                    }

                    unique_nodes.insert(*node);
                    unique_nodes.insert(dst);
                    pb1.inc(2);

                    // If we reach the desired number of unique nodes we can stop the iteration.
                    if unique_nodes.len() as NodeT >= nodes_number {
                        break 'outer;
                    }
                }
            }
        }

        pb1.finish();

        let edges_bitmap =
            RoaringTreemap::from_iter(unique_nodes.iter().progress_with(pb2).flat_map(|src| {
                let (min_edge_id, max_edge_id) = self.get_minmax_edge_ids_by_source_node_id(src);
                (min_edge_id..max_edge_id)
                    .filter(|edge_id| {
                        unique_nodes
                            .contains(self.get_unchecked_destination_node_id_by_edge_id(*edge_id))
                    })
                    .collect::<Vec<EdgeT>>()
            }));

        Graph::build_graph(
            edges_bitmap
                .iter()
                .progress_with(pb3)
                .map(|edge_id| Ok(self.get_edge_quadruple(edge_id))),
            edges_bitmap.len() as usize,
            self.nodes.clone(),
            self.node_types.clone(),
            self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
            self.directed,
            true,
            format!("{} subgraph", self.name.clone()),
            false,
            self.has_edge_types(),
            self.has_weights(),
            true,
            self.has_selfloops(),
            true,
        )
    }

    /// Returns train and test graph following kfold validation scheme.
    ///
    /// The edges are splitted into k chunks. The k_index-th chunk is used to build
    /// the validation graph, all the other edges create the training graph.
    ///
    /// # Arguments
    ///
    /// * `edge_types`: Option<Vec<String>> - Edge types to be selected when computing the folds
    ///         (All the edge types not listed here will be always be used in the training set).
    /// * `k`: u64 - The number of folds.
    /// * `k_index`: u64 - Which fold to use for the validation.
    /// * `random_state`: NodeT - The random_state (seed) to use for the holdout,
    /// * `verbose`: bool - whether to show the loading bar.
    ///
    pub fn kfold(
        &self,
        k: EdgeT,
        k_index: u64,
        edge_types: Option<Vec<Option<String>>>,
        random_state: EdgeT,
        verbose: bool,
    ) -> Result<(Graph, Graph), String> {
        if k == 1 {
            return Err(String::from("Cannot do a k-fold with only one fold."));
        }
        if k_index >= k {
            return Err(String::from(
                "The index of the k-fold must be strictly less than the number of folds.",
            ));
        }

        // If edge types is not None, to compute the chunks only use the edges
        // of the chosen edge_types
        let mut indices = if let Some(ets) = edge_types {
            if ets.is_empty() {
                return Err(String::from(
                    "Required edge types must be a non-empty list.",
                ));
            }

            let edge_type_ids = self
                .get_edge_type_ids_by_edge_type_names(ets)?
                .into_iter()
                .collect::<HashSet<Option<EdgeTypeT>>>();

            self.iter_edges_with_type_ids(self.directed)
                .filter_map(|(edge_id, _, _, edge_type)| {
                    if !edge_type_ids.contains(&edge_type) {
                        return None;
                    }
                    Some(edge_id)
                })
                .collect::<Vec<EdgeT>>()
        } else {
            self.iter_edge_ids(self.directed)
                .map(|(edge_id, _, _)| edge_id)
                .collect::<Vec<EdgeT>>()
        };

        if k >= indices.len() as EdgeT {
            return Err(String::from(
                "Cannot do a number of k-fold greater than the number of available edges.",
            ));
        }

        // if the graph has 8 edges and k = 3
        // we want the chunks sized to be:
        // 3, 3, 2

        // if the graph has 4 edges and k = 3
        // we want the chunks sized to be:
        // 2, 1, 1

        // shuffle the indices
        let mut rng = SmallRng::seed_from_u64(random_state ^ SEED_XOR as EdgeT);
        indices.shuffle(&mut rng);
        // Get the k_index-th chunk
        let chunk_size = indices.len() as f64 / k as f64;
        let start = (k_index as f64 * chunk_size).ceil() as EdgeT;
        let end = std::cmp::min(
            indices.len() as EdgeT,
            (((k_index + 1) as f64) * chunk_size).ceil() as EdgeT,
        );
        let chunk =
            RoaringTreemap::from_iter(indices[start as usize..end as usize].iter().cloned());
        // Create the two graphs
        self.edge_holdout(
            random_state,
            end - start,
            false,
            |edge_id, _, _, _| chunk.contains(edge_id),
            verbose,
            true,
            self.has_selfloops(),
        )
    }
}
