use super::types::*;
use super::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

/// # Human readable report of the properties of the graph
impl Graph {
    /// Returns report relative to the graph metrics
    ///
    /// The report includes the following metrics by default:
    /// * Name of the graph
    /// * Whether the graph is directed or undirected
    /// * Number of singleton nodes
    /// * Number of nodes
    ///     - If the graph has nodes, we also compute:
    ///         * Minimum unweighted node degree
    ///         * Maximum unweighted node degree
    ///         * Unweighted node degree mean
    /// * Number of edges
    /// * Number of self-loops
    /// * Number of singleton with self-loops
    /// * Whether the graph is a multigraph
    /// * Number of parallel edges
    /// * Number of directed edges
    ///     - If the graph has edges, we also compute:
    ///         * Rate of self-loops
    /// * Whether the graph has weighted edges
    ///     - If the graph has weights, we also compute:
    ///         * Minimum weighted node degree
    ///         * Maximum weighted node degree
    ///         * Weighted node degree mean
    ///         * The total edge weights
    /// * Whether the graph has node types
    ///     - If the graph has node types, we also compute:
    ///         * Whether the graph has singleton node types
    ///         * The number of node types
    ///         * The number of nodes with unknown node types
    ///         * The number of nodes with known node types
    /// * Whether the graph has edge types
    ///     - If the graph has edge types, we also compute:
    ///         * Whether the graph has singleton edge types
    ///         * The number of edge types
    ///         * The number of edges with unknown edge types
    ///         * The number of edges with known edge types
    ///
    /// On request, since it takes more time to compute it, the method also provides:
    ///
    /// # Examples
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// graph.report();
    /// ```
    pub fn report(&self) -> HashMap<&'static str, String> {
        let mut report: HashMap<&'static str, String> = HashMap::new();

        // Adding the default metrics
        report.insert("name", (*self.name).clone());
        report.insert("directed", self.is_directed().to_string());
        report.insert("number_of_nodes", self.get_number_of_nodes().to_string());
        report.insert(
            "singleton_number_of_nodes",
            self.get_number_of_singleton_nodes().to_string(),
        );
        if self.has_nodes() {
            report.insert("density", self.get_density().unwrap().to_string());
            report.insert(
                "minimum_node_degree",
                self.get_minimum_node_degree().unwrap().to_string(),
            );
            report.insert(
                "maximum_node_degree",
                self.get_maximum_node_degree().unwrap().to_string(),
            );
            report.insert(
                "unweighted_node_degrees_mean",
                self.get_node_degrees_mean().unwrap().to_string(),
            );
        }
        report.insert(
            "directed_number_of_edges",
            self.get_number_of_directed_edges().to_string(),
        );
        report.insert(
            "selfloops_number",
            self.get_number_of_selfloops().to_string(),
        );
        report.insert(
            "singleton_nodes_with_selfloops_number",
            self.get_number_of_singleton_nodes_with_selfloops()
                .to_string(),
        );
        report.insert("multigraph", self.is_multigraph().to_string());
        report.insert(
            "parallel_number_of_edges",
            self.get_number_of_parallel_edges().to_string(),
        );
        if self.has_edges() {
            report.insert(
                "selfloops_rate",
                self.get_selfloop_nodes_rate().unwrap().to_string(),
            );
        }
        report.insert("has_edge_weights", self.has_edge_weights().to_string());
        if self.has_edge_weights() {
            report.insert(
                "minimum_weighted_node_degree",
                self.get_weighted_minimum_node_degree()
                    .clone()
                    .unwrap()
                    .to_string(),
            );
            report.insert(
                "maximum_weighted_node_degree",
                self.get_weighted_maximum_node_degree()
                    .clone()
                    .unwrap()
                    .to_string(),
            );
            report.insert(
                "unweighted_node_degrees_mean",
                self.get_weighted_node_degrees_mean()
                    .clone()
                    .unwrap()
                    .to_string(),
            );
            report.insert(
                "total_edge_weights",
                self.get_total_edge_weights().clone().unwrap().to_string(),
            );
        }
        report.insert("has_node_types", self.has_node_types().to_string());
        if self.has_node_types() {
            report.insert(
                "has_singleton_node_types",
                self.has_singleton_node_types().unwrap().to_string(),
            );
            report.insert(
                "number_of_node_types",
                self.get_number_of_node_types().unwrap().to_string(),
            );
            report.insert(
                "unknown_number_of_node_types",
                self.get_number_of_unknown_node_types().unwrap().to_string(),
            );
            report.insert(
                "known_number_of_node_types",
                self.get_number_of_known_node_types().unwrap().to_string(),
            );
        }
        report.insert("has_edge_types", self.has_edge_types().to_string());
        if self.has_edge_types() {
            report.insert(
                "has_singleton_edge_types",
                self.has_singleton_edge_types().unwrap().to_string(),
            );
            report.insert(
                "number_of_edge_types",
                self.get_number_of_edge_types().unwrap().to_string(),
            );
            report.insert(
                "unknown_number_of_edge_types",
                self.get_number_of_unknown_edge_types().unwrap().to_string(),
            );
            report.insert(
                "known_number_of_edge_types",
                self.get_number_of_known_edge_types().unwrap().to_string(),
            );
        }

        report
    }

    fn shared_components_number(&self, nodes_components: &[NodeT], other: &Graph) -> NodeT {
        other
            .iter_node_names_and_node_type_names()
            .filter_map(
                |(_, node_name, _, _)| match self.get_node_id_from_node_name(&node_name) {
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
                    self.get_node_id_from_node_name(&src_name),
                    self.get_node_id_from_node_name(&dst_name),
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
    /// * `other`: &Graph - graph to create overlap report with.
    /// * `verbose`: Option<bool> - Whether to shor the loading bars.
    pub fn overlap_textual_report(&self, other: &Graph, verbose: Option<bool>) -> Result<String> {
        // Checking if overlap is allowed
        self.validate_operator_terms(other)?;
        // Get overlapping nodes
        let overlapping_number_of_nodes = self
            .iter_node_names_and_node_type_names()
            .filter(|(_, node_name, _, node_type)| {
                other.has_node_name_and_node_type_name(node_name, node_type.clone())
            })
            .count();
        // Get overlapping edges
        let overlapping_number_of_edges = if other.has_edge_types() && self.has_edge_types() {
            self.par_iter_directed_edge_node_names_and_edge_type_name()
                .filter(|(_, _, src_name, _, dst_name, _, edge_type_name)| {
                    other.has_edge_from_node_names_and_edge_type_name(
                        src_name,
                        dst_name,
                        edge_type_name.as_deref(),
                    )
                })
                .count()
        } else {
            self.par_iter_directed_edges()
                .filter(|(_, _, src_name, _, dst_name)| {
                    other.has_edge_from_node_names(src_name, dst_name)
                })
                .count()
        };
        // Get number of overlapping components
        let first_nodes_components = self.get_node_connected_component_ids(verbose);
        let second_nodes_components = other.get_node_connected_component_ids(verbose);
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
            true => self.get_number_of_directed_edges(),
            false => self.get_number_of_undirected_edges(),
        };
        let second_edges = match other.directed {
            true => other.get_number_of_directed_edges(),
            false => other.get_number_of_undirected_edges(),
        };
        // Building up the report
        Ok(format!(
            concat!(
                "The graph {first_graph} and the graph {second_graph} share {number_of_nodes} nodes and {number_of_edges} edges. ",
                "By percent, {first_graph} shares {first_node_percentage:.2}% ({number_of_nodes} out of {first_nodes}) of its nodes and {first_edge_percentage:.2}% ({number_of_edges} out of {first_edges}) of its edges with {second_graph}. ",
                "{second_graph} shares {second_node_percentage:.2}% ({number_of_nodes} out of {second_nodes}) of its nodes and {second_edge_percentage:.2}% ({number_of_edges} out of {second_edges}) of its edges with {first_graph}. ",
                "Nodes from {first_graph} appear in {first_components_statement} components of {second_graph}{first_merged_components_statement}. ",
                "Similarly, nodes from {second_graph} appear in {second_components_statement} components of {first_graph}{second_merged_components_statement}. ",
            ),
            first_graph=self.get_name(),
            second_graph=other.get_name(),
            number_of_nodes=overlapping_number_of_nodes,
            number_of_edges=overlapping_number_of_edges,
            first_nodes=self.get_number_of_nodes(),
            second_nodes=other.get_number_of_nodes(),
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
                    ": of these, {number_of_edges} connected by edges of {first_graph}",
                    first_graph=self.name,
                    number_of_edges= match second_merged_components_number {
                        d if d==0=>"none are".to_owned(),
                        d if d==1=>"one is".to_owned(),
                        d if d==second_components_number=>"all components are".to_owned(),
                        _ => format!("{} components are", second_merged_components_number)
                    })
                },
            second_merged_components_statement = match first_components_number > 1 {
                false=>"".to_owned(),
                true=>format!(
                    ": of these, {number_of_edges} connected by edges of {second_graph}",
                    second_graph=other.name,
                    number_of_edges= match first_merged_components_number {
                        d if d==0=>"none are".to_owned(),
                        d if d==1=>"one is".to_owned(),
                        d if d==first_components_number=>"all components are".to_owned(),
                        _ => format!("{} components are", first_merged_components_number)
                    })
                },
            first_node_percentage=100.0*(overlapping_number_of_nodes as f64 / self.get_number_of_nodes() as f64),
            second_node_percentage=100.0*(overlapping_number_of_nodes as f64 / other.get_number_of_nodes() as f64),
            first_edge_percentage=100.0*(overlapping_number_of_edges as f64 / first_edges as f64),
            second_edge_percentage=100.0*(overlapping_number_of_edges as f64 / second_edges as f64),
        ))
    }

    /// Return human-readable html report of the given node.
    ///
    /// The report, by default, is rendered using html.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_id(&self, node_id: NodeT) -> Result<String> {
        self.validate_node_id(node_id)?;
        let mut partial_reports: Vec<String> = Vec::new();
        let node_name = unsafe { self.get_unchecked_node_name_from_node_id(node_id) };

        partial_reports.push(
            if unsafe { self.is_unchecked_singleton_from_node_id(node_id) } {
                match self.get_number_of_singleton_nodes() {
                    1 => format!(
                        concat!("The given node {} is the only singleton node of the graph."),
                        node_name
                    ),
                    singleton_number_of_nodes => {
                        format!(
                            concat!("The given node {} is one of {} singleton nodes."),
                            node_name, singleton_number_of_nodes
                        )
                    }
                }
            } else if unsafe { self.is_unchecked_singleton_with_selfloops_from_node_id(node_id) } {
                match self.get_number_of_singleton_nodes_with_selfloops() {
                    1 => format!(
                        concat!(
                        "The given node {} is the only singleton node with self-loops in the graph."
                    ),
                        node_name
                    ),
                    singleton_nodes_with_selfloops_number => {
                        format!(
                            concat!(
                                "The given node {} is one of {} singleton nodes with selfloops."
                            ),
                            node_name, singleton_nodes_with_selfloops_number
                        )
                    }
                }
            } else if unsafe { self.is_unchecked_trap_node_from_node_id(node_id) } {
                match self.get_number_of_trap_nodes() {
                    1 => format!(
                        concat!("The given node {} is the only trap node in the graph."),
                        node_name
                    ),
                    trap_number_of_nodes => {
                        format!(
                            concat!("The given node {} is one of {} trap nodes in the graph."),
                            node_name, trap_number_of_nodes
                        )
                    }
                }
            } else {
                format!(
                    concat!("The given node {} has degree {}"),
                    node_name,
                    unsafe { self.get_unchecked_node_degree_from_node_id(node_id) }
                )
            },
        );

        Ok(partial_reports.join(""))
    }

    /// Return human-readable html report of the given node.
    ///
    /// The report, by default, is rendered using html.
    ///
    /// # Arguments
    /// * `node_name`: &str - Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_name(&self, node_name: &str) -> Result<String> {
        self.get_node_id_from_node_name(node_name)
            .and_then(|node_id| self.get_node_report_from_node_id(node_id))
    }

    /// Returns html formatting for the given node name URLs.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node ID to query for.
    /// * `minimum_node_degree`: NodeT - The minimum node degree to show the node degree information. This parameter is available because in some use cases (e.g. the stars report) the degree is extremely redoundant.
    /// * `show_node_type`: bool - Whether to display the node nodes type.
    ///
    /// # Safety
    /// This method will cause an out of bound if the given node ID does not exist.
    pub(crate) unsafe fn get_unchecked_succinct_node_attributes_description(
        &self,
        node_id: NodeT,
        minimum_node_degree: NodeT,
        show_node_type: bool,
    ) -> String {
        let node_type = if show_node_type && self.has_node_types() {
            match self.get_unchecked_node_type_names_from_node_id(node_id) {
                Some(node_type_names) => match node_type_names.len() {
                    0 => unreachable!("A node cannot have an empty list of node types, as that case should be None."),
                    1 => Some(format!(
                        "node type {}",
                        get_node_type_source_html_url_from_node_type_name(
                            node_type_names.first().unwrap().as_ref()
                        )
                    )),
                    _ => Some(format!(
                        "node types {}",
                        get_unchecked_formatted_list(
                            node_type_names
                                .iter()
                                .map(|node_type_name| {
                                    get_node_type_source_html_url_from_node_type_name(
                                        node_type_name,
                                    )
                                })
                                .collect::<Vec<_>>()
                                .as_ref(),
                                Some(5)
                        )
                    )),
                },
                None => Some("unknown node type".to_string()),
            }
        } else {
            None
        };
        let node_degree = self.get_unchecked_node_degree_from_node_id(node_id);
        let node_degree = if node_degree <= minimum_node_degree {
            None
        } else {
            Some(format!(
                "degree {}",
                to_human_readable_high_integer(node_degree as usize)
            ))
        };

        // If any of the terms was given we build the output description
        let description = if node_degree.is_some() || node_type.is_some() {
            let node_degree_is_some = node_degree.is_some();
            format!(
                "{node_degree}{join_term}{node_type}",
                node_degree = node_degree.unwrap_or_else(|| "".to_string()),
                join_term = if node_degree_is_some && node_type.is_some() {
                    " and "
                } else {
                    ""
                },
                node_type = node_type.unwrap_or_else(|| "".to_string())
            )
        } else {
            "".to_string()
        };
        description
    }

    /// Returns html formatting for the given node name URLs.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node ID to query for.
    /// * `minimum_node_degree`: NodeT - The minimum node degree to show the node degree information. This parameter is available because in some use cases (e.g. the stars report) the degree is extremely redoundant.
    /// * `show_node_type`: bool - Whether to display the node nodes type.
    ///
    /// # Safety
    /// This method will cause an out of bound if the given node ID does not exist.
    pub(crate) unsafe fn get_unchecked_succinct_node_description(
        &self,
        node_id: NodeT,
        minimum_node_degree: NodeT,
        show_node_type: bool,
    ) -> String {
        let node_name = get_node_source_html_url_from_node_name(
            self.get_unchecked_node_name_from_node_id(node_id).as_ref(),
        );
        let description = self.get_unchecked_succinct_node_attributes_description(
            node_id,
            minimum_node_degree,
            show_node_type,
        );
        let description = if description.is_empty() {
            description
        } else {
            format!(" ({})", description)
        };
        format!(
            "{node_name}{description}",
            node_name = node_name,
            description = description
        )
    }

    /// Returns html formatting for the given node type id attributes.
    ///
    /// # Arguments
    /// * `node_type_id`: NodeTypeT - Node ID to query for.
    /// * `local_total`: Option<NodeT> - The total amount of nodes in this local area. If None, the number of nodes is used.
    /// * `local_count`: Option<NodeT> - The total amount of nodes of this type in this local area. If None, the amount of nodes of this type in the graph is used.
    ///
    /// # Safety
    /// This method will cause an out of bound if the given node type ID does not exist.
    pub(crate) unsafe fn get_unchecked_succinct_node_type_attributes_description(
        &self,
        node_type_id: NodeTypeT,
        local_total: Option<NodeT>,
        local_count: Option<NodeT>,
    ) -> String {
        let total_nodes = local_total.unwrap_or_else(|| self.get_number_of_nodes());
        let number_of_nodes = local_count
            .unwrap_or_else(|| self.get_unchecked_number_of_nodes_from_node_type_id(node_type_id));
        if number_of_nodes > 1 {
            let percentage_of_nodes = number_of_nodes as f64 / total_nodes as f64 * 100.0;
            format!(
                "{number_of_nodes} nodes{percentage_of_nodes}",
                number_of_nodes = to_human_readable_high_integer(number_of_nodes as usize),
                percentage_of_nodes = if percentage_of_nodes >= 0.01 && total_nodes > 10 {
                    format!(
                        ", {percentage_of_nodes:.2}%",
                        percentage_of_nodes = percentage_of_nodes
                    )
                } else {
                    "".to_string()
                }
            )
        } else {
            "".to_string()
        }
    }

    /// Returns html formatting for the given node type id.
    ///
    /// # Arguments
    /// * `node_type_id`: NodeTypeT - Node ID to query for.
    /// * `local_total`: Option<NodeT> - The total amount of nodes in this local area. If None, the number of nodes is used.
    /// * `local_count`: Option<NodeT> - The total amount of nodes of this type in this local area. If None, the amount of nodes of this type in the graph is used.
    ///
    /// # Safety
    /// This method will cause an out of bound if the given node type ID does not exist.
    pub(crate) unsafe fn get_unchecked_succinct_node_type_description(
        &self,
        node_type_id: NodeTypeT,
        local_total: Option<NodeT>,
        local_count: Option<NodeT>,
    ) -> String {
        let node_type_name = get_node_type_source_html_url_from_node_type_name(
            self.get_node_type_name_from_node_type_id(node_type_id)
                .unwrap()
                .as_ref(),
        );

        let description = self.get_unchecked_succinct_node_type_attributes_description(
            node_type_id,
            local_total,
            local_count,
        );

        let description = if description.is_empty() {
            description
        } else {
            format!(" ({})", description)
        };
        format!(
            "{node_type_name}{description}",
            node_type_name = node_type_name,
            description = description
        )
    }

    /// Returns html formatting for the given edge type id attributes.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - edge ID to query for.
    /// * `local_total`: Option<EdgeT> - The total amount of edges in this local area. If None, the directed number of edges is used.
    /// * `local_count`: Option<EdgeT> - The total amount of edges of this type in this local area. If None, the amount of edges of this type in the graph is used.
    ///
    /// # Safety
    /// This method will cause an out of bound if the given edge type ID does not exist.
    pub(crate) unsafe fn get_unchecked_succinct_edge_type_attributes_description(
        &self,
        edge_type_id: EdgeTypeT,
        local_total: Option<EdgeT>,
        local_count: Option<EdgeT>,
    ) -> String {
        let total_edges = local_total.unwrap_or_else(|| self.get_number_of_directed_edges());
        let number_of_edges = local_count
            .unwrap_or_else(|| self.get_unchecked_number_of_edges_from_edge_type_id(edge_type_id));
        if number_of_edges > 1 {
            let percentage_of_edges = number_of_edges as f64 / total_edges as f64 * 100.0;
            format!(
                "{number_of_edges} edges{percentage_of_edges}",
                number_of_edges = to_human_readable_high_integer(number_of_edges as usize),
                percentage_of_edges = if percentage_of_edges >= 0.01 && total_edges > 10 {
                    format!(
                        ", {percentage_of_edges:.2}%",
                        percentage_of_edges = percentage_of_edges
                    )
                } else {
                    "".to_string()
                }
            )
        } else {
            "".to_string()
        }
    }

    /// Returns html formatting for the given edge type id.
    ///
    /// # Arguments
    /// * `edge_type_id`: EdgeTypeT - edge ID to query for.
    /// * `local_total`: Option<EdgeT> - The total amount of edges in this local area. If None, the directed number of edges is used.
    /// * `local_count`: Option<EdgeT> - The total amount of edges of this type in this local area. If None, the amount of edges of this type in the graph is used.
    ///
    /// # Safety
    /// This method will cause an out of bound if the given edge type ID does not exist.
    pub(crate) unsafe fn get_unchecked_succinct_edge_type_description(
        &self,
        edge_type_id: EdgeTypeT,
        local_total: Option<EdgeT>,
        local_count: Option<EdgeT>,
    ) -> String {
        let edge_type_name = get_edge_type_source_html_url_from_edge_type_name(
            self.get_edge_type_name_from_edge_type_id(edge_type_id)
                .unwrap()
                .as_ref(),
        );

        let description = self.get_unchecked_succinct_edge_type_attributes_description(
            edge_type_id,
            local_total,
            local_count,
        );

        let description = if description.is_empty() {
            description
        } else {
            format!(" ({})", description)
        };
        format!(
            "{edge_type_name}{description}",
            edge_type_name = edge_type_name,
            description = description
        )
    }

    /// Returns html formatting for the given node name URLs.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - Node ID to query for.
    ///
    /// # Safety
    /// This method will cause an out of bound if the given edge ID does not exist.
    unsafe fn get_unchecked_succinct_edge_description(&self, edge_id: EdgeT) -> String {
        format!(
            "edge between {source_node_description:?} and {destination_node_description:?}{edge_type}",
            source_node_description = self.get_unchecked_succinct_node_description(self.get_unchecked_source_node_id_from_edge_id(edge_id), 0, true),
            destination_node_description = self.get_unchecked_succinct_node_description(self.get_unchecked_destination_node_id_from_edge_id(edge_id), 0, true),
            edge_type = if self.has_edge_types() {
                match self.get_edge_type_name_from_edge_id(edge_id).unwrap() {
                    Some(edge_type_name) => {
                        format!(
                            " with edge type {}",
                            get_edge_type_source_html_url_from_edge_type_name(edge_type_name.as_ref())
                        )
                    },
                    None => " with unknown edge type".to_string(),
                }
            } else {
                "".to_string()
            }
        )
    }

    /// Returns summary of the high level characteristics of a graph.
    ///
    /// # Implementative details
    /// The method currently supports multiple use cases, including:
    /// * Graphs without nodes
    /// * Graphs without edges
    /// * Normal graphs
    fn get_textual_report_summary(&self) -> String {
        // Create the empty list of the report paragraphs.
        let mut report = Vec::new();

        // Add the name of the graph as title if it is informative
        let name = if self.has_default_graph_name() {
            None
        } else {
            report.push(format!("<h2>{}</h2>", self.get_name()));
            Some(format!(" {}", self.get_name()))
        };

        // If the graph does not contain nodes, we limit to a very
        // simple summary.
        if !self.has_nodes() {
            report.push(format!(
                concat!(
                    "<p>The graph{name} is <b>empty</b>, that is, it has neither nodes nor edges. ",
                    "If this is unexpected, it may have happened because of a ",
                    "mis-parametrization of a filter method uphill.</p>"
                ),
                name = name.unwrap_or_else(|| "".to_string())
            ));

            return report.join("");
        }

        // Otherwise we compute a descriptor of the avilable nodes.
        let number_of_nodes = unsafe {
            match self.get_number_of_nodes() {
                1 => format!(
                    "a single node called {node_name_description}",
                    node_name_description =
                        self.get_unchecked_succinct_node_description(0, 0, true),
                ),
                number_of_nodes => format!(
                    "{number_of_nodes}{heterogeneous_nodes} nodes",
                    number_of_nodes = to_human_readable_high_integer(number_of_nodes as usize),
                    heterogeneous_nodes = match self.get_number_of_node_types() {
                        Ok(n) =>
                            if n == 1 {
                                " homogeneous"
                            } else {
                                " heterogeneous"
                            },
                        Err(_) => "",
                    },
                ),
            }
        };

        // If the graph, similarly, does not contain edges, we limit
        // to a very simple report.
        if !self.has_edges() {
            report.push(format!(
                concat!(
                    "<p>The graph{name} contains {number_of_nodes} and no edges. ",
                    "If this is unexpected, it may have happened because of a ",
                    "mis-parametrization of a filter method uphill.</p>"
                ),
                name = name.unwrap_or_else(|| "".to_string()),
                number_of_nodes = number_of_nodes
            ));

            return report.join("");
        }

        // Otherwise we compute a more comprehensive report of the edges.
        let number_of_edges = unsafe {
            match self.get_number_of_edges() {
                1 => format!(
                    "a single {edge_description}",
                    edge_description = self.get_unchecked_succinct_edge_description(0)
                ),
                number_of_edges => format!(
                    "{number_of_edges}{heterogeneous_edges} edges",
                    number_of_edges = to_human_readable_high_integer(number_of_edges as usize),
                    heterogeneous_edges = match self.get_number_of_edge_types() {
                        Ok(n) =>
                            if n == 1 {
                                " homogeneous"
                            } else {
                                " heterogeneous"
                            },
                        Err(_) => "",
                    },
                ),
            }
        };

        // And if the graph is undirected, we have some more high efficiency
        // algorithms that we may want to use.
        // We compute the connected components, if the speed-ups relative to
        // the destinations are enabled (as it may be too slow otherwise on
        // some instances. Similarly, if the graph has less than 1M nodes we
        // also compute the connected components as it should be quite immediate.
        let connected_components = if !self.is_directed() {
            format!("{} ", self.get_report_of_connected_components())
        } else {
            "".to_string()
        };

        // And put the report summary line togheter.
        report.push(format!(
            concat!(
                "<p>",
                "The {directionality} {multigraph}graph{name} has {number_of_nodes} and {number_of_edges}. ",
                "{connected_components}",
                "The RAM requirements for the nodes and edges data structures are {ram_nodes} and {ram_edges} respectively.",
                "</p>"
            ),
            directionality = if self.is_directed() {
                "directed"
            } else {
                "undirected"
            },
            multigraph = if self.is_multigraph() {
                "multi"
            } else {
                ""
            },
            name = name.unwrap_or_else(|| "".to_string()),
            number_of_nodes = number_of_nodes,
            number_of_edges = number_of_edges,
            connected_components=connected_components,
            ram_nodes = self.get_nodes_total_memory_requirement_human_readable(),
            ram_edges = self.get_edges_total_memory_requirement_human_readable()
        ));

        report.join("")
    }

    /// Returns report on the unweighted node degree centrality.
    ///
    /// # Safety
    /// This method may cause a panic when called on a graph with no edges.
    unsafe fn get_node_degree_centrality_report(&self) -> String {
        format!(
            concat!(
                "<h3>Degree centrality</h3>",
                "<p>The minimum node degree is {minimum_node_degree}, the maximum node degree is {maximum_node_degree}, ",
                "the mode degree is {mode_node_degree}, the mean degree is {mean_node_degree:.2} and the node degree median is {node_degree_median}.</p>",
                "<p>The nodes with the highest degree centrality are {list_of_most_central_nodes}.</p>"
            ),
            minimum_node_degree = to_human_readable_high_integer(self.get_minimum_node_degree().unwrap() as usize),
            maximum_node_degree = to_human_readable_high_integer(self.get_maximum_node_degree().unwrap() as usize),
            mode_node_degree = to_human_readable_high_integer(self.get_node_degrees_mode().unwrap() as usize),
            mean_node_degree = self.get_node_degrees_mean().unwrap(),
            node_degree_median = to_human_readable_high_integer(self.get_node_degrees_median().unwrap() as usize),
            list_of_most_central_nodes = get_unchecked_formatted_list(
                self.get_top_k_central_node_ids(5).unwrap()
                    .into_iter()
                    .filter(|node_id| {
                        self.get_unchecked_node_degree_from_node_id(*node_id) > 0
                    })
                    .map(|node_id| {
                        self.get_unchecked_succinct_node_description(node_id, 0, true)
                    })
                    .collect::<Vec<_>>()
                    .as_ref(),
                    None
            )
        )
    }

    /// Returns report on the oddities detected within the graph.
    fn get_report_of_connected_components(&self) -> String {
        let (components_number, minimum_component_size, maximum_component_size) =
            self.get_number_of_connected_components(None);
        if components_number == 1 {
            return concat!(
                "The graph is connected, that is, it is composed of a single connected component that includes all nodes and edges."
            ).to_string();
        }
        format!(
            concat!(
                "The graph contains {} connected components{}, with the largest one containing {} nodes and the smallest one containing {}.",
            ),
            to_human_readable_high_integer(components_number as usize),
            match self.get_number_of_disconnected_nodes() {
                0 => "".to_string(),
                disconnected_number_of_nodes => format!(
                    " (of which {} are disconnected nodes)",
                    to_human_readable_high_integer(disconnected_number_of_nodes as usize)
                )
            },
            to_human_readable_high_integer(maximum_component_size as usize),
            if minimum_component_size == 1 {
                "a single node".to_string()
            } else {
                format!(
                    "{} nodes",
                    to_human_readable_high_integer(minimum_component_size as usize)
                )
            },
        )
    }

    /// Returns report on the provided tree-like oddity list.
    ///
    /// # Arguments
    /// * `header_type`: &str - Type of header to use for this section.
    /// * `oddity_name`: &str - Name of oddity.
    /// * `plural_oddity_name`: &str - Name of oddity.
    /// * `oddity_description`: &str - Description of oddity.
    /// * `number_of_oddities`: NodeT - Number of the oddities.
    /// * `number_of_involved_nodes`: NodeT - Number of involved nodes.
    /// * `number_of_involved_edges`: EdgeT - Number of involved edges.
    /// * `maximum_number_of_involved_nodes`: NodeT - Number of nodes involved in the largest oddity of this type.
    /// * `maximum_number_of_involved_edges`: EdgeT - Number of edges involved in the largest oddity of this type.
    /// * `sorted`: bool - Whether to consider these oddities to be sorted.
    /// * `maximum_number_of_oddities_to_report`: Option<usize> - Maximum number of oddities to report, by default 6.
    /// * `number_of_columns`: Option<usize> - Number of columns to use for the report visualization. By default 1.
    /// * `oddities`: impl Iterator<Item=T> - Iterator over the oddities.
    fn get_report_of_oddity<T: ToString>(
        &self,
        header_type: &str,
        oddity_name: &str,
        plural_oddity_name: &str,
        oddity_description: &str,
        number_of_oddities: NodeT,
        number_of_involved_nodes: NodeT,
        number_of_involved_edges: EdgeT,
        maximum_number_of_involved_nodes: NodeT,
        maximum_number_of_involved_edges: EdgeT,
        sorted: bool,
        maximum_number_of_oddities_to_report: Option<usize>,
        number_of_columns: Option<usize>,
        oddities: impl Iterator<Item = T>,
    ) -> String {
        if number_of_oddities == 0 {
            return "".to_string();
        }
        let maximum_number_of_oddities_to_report =
            maximum_number_of_oddities_to_report.unwrap_or(6);
        let number_of_columns = number_of_columns.unwrap_or(1);
        if oddity_name.is_empty() {
            panic!("The oddity name cannot be empty!");
        }
        if oddity_description.is_empty() {
            panic!("The oddity description cannot be empty!");
        }
        let percentage_of_involved_nodes =
            (number_of_involved_nodes as f64 / self.get_number_of_nodes() as f64) * 100.0;
        let percentage_of_involved_edges =
            (number_of_involved_edges as f64 / self.get_number_of_directed_edges() as f64) * 100.0;
        format!(
            concat!(
                "<{header_type}>{plural_oddity_name}</{header_type}>",
                "<p>",
                "{oddity_description} ",
                "We have detected {number_of_oddities_in_graph} in the graph",
                "{involved_nodes_and_edges}",
                "{maximum_involved_nodes_and_edges}.",
                "{list_description}",
                "</p>",
                "<{list_type}{column_style}>",
                "{top_oddities_description}",
                "</{list_type}>",
                "{possibly_conclusive_entry}"
            ),
            column_style = format!(
                " class=\"{}\"",
                match number_of_columns {
                    4 => "small-columns",
                    3 => "medium-columns",
                    2 => "large-columns",
                    1 => "single-column",
                    _ => unreachable!("We only support column numbers from 1 to 5."),
                }
            ),
            list_type = if sorted && number_of_oddities > 1 {
                "ol"
            } else {
                "ul"
            },
            list_description = if number_of_oddities > 1 {
                format!(
                    " The detected {lower_plural_oddity_name}{sorted_note} are:",
                    lower_plural_oddity_name = plural_oddity_name.to_lowercase(),
                    sorted_note = if sorted {
                        ", sorted by decreasing size,"
                    } else {
                        ""
                    }
                )
            } else {
                "".to_string()
            },
            header_type = header_type,
            plural_oddity_name = plural_oddity_name,
            oddity_description = oddity_description,
            number_of_oddities_in_graph = if number_of_oddities == 1 {
                format!(
                    " a single {lower_oddity_name}",
                    lower_oddity_name = oddity_name.to_lowercase()
                )
            } else {
                format!(
                    "{number_of_oddities} {lower_plural_oddity_name}",
                    number_of_oddities =
                        to_human_readable_high_integer(number_of_oddities as usize),
                    lower_plural_oddity_name = plural_oddity_name.to_lowercase(),
                )
            },
            involved_nodes_and_edges = if number_of_involved_nodes > 2 {
                format!(
                    concat!(
                        ", involving a total of {number_of_involved_nodes} nodes{percentage_of_involved_nodes}",
                        "{note_on_edges}"
                    ),
                    number_of_involved_nodes = to_human_readable_high_integer(number_of_involved_nodes as usize),
                    percentage_of_involved_nodes= if percentage_of_involved_nodes > 0.01 {
                        format!(
                            " ({percentage_of_involved_nodes:.2}%)",
                            percentage_of_involved_nodes=percentage_of_involved_nodes
                        )
                    } else {
                        "".to_string()
                    },
                    note_on_edges = if number_of_involved_edges > 0 {
                        format!(
                            " and {number_of_involved_edges} edges{percentage_of_involved_edges}",
                            number_of_involved_edges = to_human_readable_high_integer(number_of_involved_edges as usize),
                            percentage_of_involved_edges= if percentage_of_involved_edges > 0.01 {
                                format!(
                                    " ({percentage_of_involved_edges:.2}%)",
                                    percentage_of_involved_edges=percentage_of_involved_edges
                                )
                            } else {
                                "".to_string()
                            },
                        )
                    } else {
                        "".to_string()
                    }
                )
            } else {
                "".to_string()
            },
            maximum_involved_nodes_and_edges = if sorted && maximum_number_of_involved_nodes > 2 {
                format!(
                    concat!(
                        ", with the largest one involving {maximum_number_of_involved_nodes} nodes ",
                        "and {maximum_number_of_involved_edges} edges",
                    ),
                    maximum_number_of_involved_nodes = to_human_readable_high_integer(maximum_number_of_involved_nodes as usize),
                    maximum_number_of_involved_edges = to_human_readable_high_integer(maximum_number_of_involved_edges as usize),
                )
            } else {
                "".to_string()
            },
            top_oddities_description = oddities
                .take(maximum_number_of_oddities_to_report)
                .map(|oddity| format!("<li>{}</li>", oddity.to_string()))
                .join("\n"),
            possibly_conclusive_entry =
                if number_of_oddities > maximum_number_of_oddities_to_report as NodeT {
                    let remaining_oddities =
                        number_of_oddities - maximum_number_of_oddities_to_report as NodeT;
                    if remaining_oddities == 1 {
                        format!(
                            "<p>And another {lower_oddity_name}.</p>",
                            lower_oddity_name = oddity_name.to_lowercase()
                        )
                    } else {
                        format!(
                            "<p>And other {remaining_oddities} {lower_plural_oddity_name}.</p>",
                            remaining_oddities =
                                to_human_readable_high_integer(remaining_oddities as usize),
                            lower_plural_oddity_name = plural_oddity_name.to_lowercase()
                        )
                    }
                } else {
                    "".to_string()
                }
        )
    }

    /// Returns report on the provided tree-like oddity list.
    ///
    /// # Arguments
    /// * `tree_like_oddities`: Vec<DendriticTree> - Vector of oddities.
    /// * `oddity_name`: &str - Name of the oddity.
    /// * `plural_oddity_name`: &str - Name of the oddity conjugated to the plural form.
    /// * `oddity_description`: &str - Description of the oddity.
    fn get_report_of_specific_tree_like_oddities(
        &self,
        mut tree_like_oddities: Vec<DendriticTree>,
        oddity_name: &str,
        plural_oddity_name: &str,
        oddity_description: &str,
    ) -> String {
        if tree_like_oddities.is_empty() {
            "".to_string()
        } else {
            tree_like_oddities.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
            let sorted = tree_like_oddities
                .first()
                .unwrap()
                .get_number_of_involved_nodes()
                > tree_like_oddities
                    .last()
                    .unwrap()
                    .get_number_of_involved_nodes();
            self.get_report_of_oddity(
                "h4",
                oddity_name,
                plural_oddity_name,
                oddity_description,
                tree_like_oddities.len() as NodeT,
                tree_like_oddities
                    .par_iter()
                    .map(|oddity| oddity.get_number_of_involved_nodes())
                    .sum::<NodeT>(),
                tree_like_oddities
                    .par_iter()
                    .map(|oddity| oddity.get_number_of_involved_edges())
                    .sum::<EdgeT>(),
                tree_like_oddities
                    .par_iter()
                    .map(|oddity| oddity.get_number_of_involved_nodes())
                    .max()
                    .unwrap(),
                tree_like_oddities
                    .par_iter()
                    .map(|oddity| oddity.get_number_of_involved_edges())
                    .max()
                    .unwrap(),
                sorted,
                Some(6),
                Some(2),
                tree_like_oddities.into_iter(),
            )
        }
    }

    /// Returns report on the oddities detected within the graph.
    ///
    /// # Implementation details
    /// The oddities reported within this section of the report include Stars, Chains and Circles.
    /// The stars and chains, to be considered, must have at least \(10\) nodes, while the circles
    /// must have at least \(5\) nodes. When a graph does not contain a type of oddity, that section
    /// of the report is omitted. When no oddity is found, this report will be empty.
    ///
    /// # Safety
    /// This method may cause a panic when called on a graph with no edges.
    fn get_report_of_topological_oddities(&self) -> Result<Option<String>> {
        let (circles, chains, node_tuples, tree_like_oddities) = if !self.is_directed() {
            let mut circles = self.get_circles(None, None)?;
            circles.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
            let mut chains = self.get_chains(None, None)?;
            chains.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
            let mut node_tuples = self.get_node_tuples()?;
            node_tuples.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
            let tree_like_oddities = self.get_dendritic_trees()?;
            (circles, chains, node_tuples, tree_like_oddities)
        } else {
            (Vec::new(), Vec::new(), Vec::new(), Vec::new())
        };

        let mut isomorphic_node_groups: Vec<Vec<NodeT>> = self
            .get_isomorphic_node_ids(None, None, None)
            .unwrap_or_else(|_| Vec::new());

        isomorphic_node_groups.sort_unstable_by(|group1, group2| unsafe {
            (self.get_unchecked_node_degree_from_node_id(group2[0]) as usize * group2.len()).cmp(
                &(self.get_unchecked_node_degree_from_node_id(group1[0]) as usize * group1.len()),
            )
        });

        let mut isomorphic_edge_groups: Vec<Vec<[NodeT; 2]>> = self
            .get_isomorphic_edge_node_ids(None, None, None, None)
            .unwrap_or_else(|_| Vec::new());

        isomorphic_edge_groups.sort_unstable_by(|group1, group2| unsafe {
            ((self.get_unchecked_node_degree_from_node_id(group2[0][0]) as usize
                + self.get_unchecked_node_degree_from_node_id(group2[0][1]) as usize)
                * group2.len())
            .cmp(
                &(self.get_unchecked_node_degree_from_node_id(group1[0][0]) as usize
                    + self.get_unchecked_node_degree_from_node_id(group1[0][1]) as usize
                        * group1.len()),
            )
        });

        // If the graph does not contain any oddity, we do not prepare a report.
        if isomorphic_edge_groups.is_empty()
            && isomorphic_node_groups.is_empty()
            && circles.is_empty()
            && chains.is_empty()
            && node_tuples.is_empty()
            && tree_like_oddities.is_empty()
            && !self.has_disconnected_nodes()
        {
            return Ok(None);
        }

        let number_of_singleton_nodes = self.get_number_of_singleton_nodes();
        let singleton_nodes_description = self.get_report_of_oddity(
            "h4",
            "Singleton node",
            "Singleton nodes",
            concat!("A singleton node is a node disconnected from all other nodes."),
            number_of_singleton_nodes,
            number_of_singleton_nodes,
            0,
            1,
            0,
            false,
            Some(15),
            Some(4),
            self.iter_singleton_node_ids().map(|node_id| unsafe {
                format!(
                    "<p>{}</p>",
                    self.get_unchecked_succinct_node_description(node_id, 0, true)
                )
            }),
        );

        let number_of_singleton_nodes_with_selfloops =
            self.get_number_of_singleton_nodes_with_selfloops();
        let number_of_edges_involved_in_singleton_with_selfloops = self
            .par_iter_singleton_nodes_with_selfloops_node_ids()
            .map(|node_id| unsafe { self.get_unchecked_node_degree_from_node_id(node_id) as EdgeT })
            .sum::<EdgeT>();
        let maximum_number_of_edges_in_a_singleton_with_selfloop =
            self.par_iter_singleton_nodes_with_selfloops_node_ids()
                .map(|node_id| unsafe { self.get_unchecked_node_degree_from_node_id(node_id) })
                .max()
                .unwrap_or(0) as EdgeT;

        let singleton_nodes_with_selfloops_description = self.get_report_of_oddity(
            "h4",
            "Singleton node with self-loops",
            "Singleton nodes with self-loops",
            concat!(
                "A singleton node with self-loops is a node disconnected ",
                "from all other nodes except itself."
            ),
            number_of_singleton_nodes_with_selfloops,
            number_of_singleton_nodes_with_selfloops,
            number_of_edges_involved_in_singleton_with_selfloops,
            1,
            maximum_number_of_edges_in_a_singleton_with_selfloop,
            false,
            Some(15),
            Some(4),
            self.iter_singleton_nodes_with_selfloops_node_ids()
                .map(|node_id| unsafe {
                    format!(
                        "<p>{}</p>",
                        self.get_unchecked_succinct_node_description(node_id, 1, true)
                    )
                }),
        );

        let trap_nodes_description = if self.has_trap_nodes() {
            self.get_report_of_oddity(
                "h4",
                "Trap node",
                "Trap nodes",
                concat!(
                    "In a directed graph, a trap node is a node that blocks ",
                    "a random walk, having a some inbound edges and no outbound edges."
                ),
                self.get_number_of_trap_nodes(),
                self.get_number_of_trap_nodes(),
                self.par_iter_directed_edge_node_ids()
                    .filter(|(_, src, dst)| unsafe{
                    self.is_unchecked_trap_node_from_node_id(*dst)
                }).count() as EdgeT,
                1,
                0,
                false,
                Some(15),
                Some(4),
                self.iter_trap_node_ids()
                    .map(|node_id| unsafe {
                        format!(
                            "<p>{}</p>",
                            self.get_unchecked_succinct_node_description(node_id, 1, true)
                        )
                    }),
            )
        } else {
            "".to_string()
        };

        let trap_nodes_with_selfloop_description = if self.has_trap_nodes() {
            self.get_report_of_oddity(
                "h4",
                "Singleton node with self-loops",
                "Singleton nodes with self-loops",
                concat!(
                    "In a directed graph, a trap node with selfloops is a node that traps ",
                    "a random walk, having a some inbound edges and only outbound edges to itself. ",
                    "Therefore, the random walk can continue ad infinitum, but forever trapped in the same node."
                ),
                self.get_number_of_trap_nodes(),
                self.get_number_of_trap_nodes(),
                self.par_iter_directed_edge_node_ids()
                    .filter(|(_, src, dst)| unsafe{
                    self.is_unchecked_trap_node_with_selfloops_from_node_id(*dst)
                }).count() as EdgeT,
                1,
                0,
                false,
                Some(15),
                Some(4),
                self.iter_trap_node_with_selfloops_ids()
                    .map(|node_id| unsafe {
                        format!(
                            "<p>{}</p>",
                            self.get_unchecked_succinct_node_description(node_id, 1, true)
                        )
                    }),
            )
        } else {
            "".to_string()
        };

        let number_of_circles = circles.len() as NodeT;
        let number_of_nodes_involved_in_circles = circles.iter().map(|circle| circle.len()).sum();
        let number_of_edges_involved_in_circles =
            (number_of_nodes_involved_in_circles + number_of_circles) as EdgeT;
        let maximum_number_of_nodes_in_a_circle =
            circles.iter().map(|circle| circle.len()).max().unwrap_or(0);
        let maximum_number_of_edges_in_a_circle = maximum_number_of_nodes_in_a_circle as EdgeT + 1;
        let circles_description = self.get_report_of_oddity(
            "h4",
            "Circle",
            "Circles",
            concat!(
                "A circle is a connected component composed ",
                "exclusively of nodes with unique degree 2, ",
                "that is ignoring self-loop and multi-edges."
            ),
            number_of_circles,
            number_of_nodes_involved_in_circles,
            number_of_edges_involved_in_circles,
            maximum_number_of_nodes_in_a_circle,
            maximum_number_of_edges_in_a_circle,
            true,
            Some(10),
            Some(3),
            circles.into_iter(),
        );

        let number_of_chains = chains.len() as NodeT;
        let number_of_nodes_involved_in_chains =
            chains.iter().map(|chain| chain.len()).sum::<NodeT>();
        let number_of_edges_involved_in_chains =
            number_of_nodes_involved_in_chains.saturating_sub(1) as EdgeT;
        let maximum_number_of_nodes_in_a_chain =
            chains.iter().map(|chain| chain.len()).max().unwrap_or(0);
        let maximum_number_of_edges_in_a_chain =
            maximum_number_of_nodes_in_a_chain.saturating_sub(1) as EdgeT;
        let chains_description = self.get_report_of_oddity(
            "h4",
            "Chain",
            "Chains",
            concat!(
                "A chain is a path of nodes with unique degree 2, ",
                "that is ignoring self-loop and multi-edges, ",
                "connecting two strongly connected components of the graph."
            ),
            number_of_chains,
            number_of_nodes_involved_in_chains,
            number_of_edges_involved_in_chains,
            maximum_number_of_nodes_in_a_chain,
            maximum_number_of_edges_in_a_chain,
            true,
            Some(10),
            Some(3),
            chains.into_iter(),
        );

        let number_of_node_tuples = node_tuples.len() as NodeT;
        let number_of_nodes_involved_in_node_tuples = number_of_node_tuples * 2;
        let number_of_edges_involved_in_node_tuples = number_of_node_tuples as EdgeT;
        let maximum_number_of_nodes_in_a_node_tuple = 2;
        let maximum_number_of_edges_in_a_node_tuple = 1;
        let node_tuples_description = self.get_report_of_oddity(
            "h4",
            "Node tuple",
            "Node tuples",
            concat!("A node tuple is a connected component composed of two nodes."),
            number_of_node_tuples,
            number_of_nodes_involved_in_node_tuples,
            number_of_edges_involved_in_node_tuples,
            maximum_number_of_nodes_in_a_node_tuple,
            maximum_number_of_edges_in_a_node_tuple,
            false,
            Some(15),
            Some(3),
            node_tuples.into_iter(),
        );

        // ================================
        // Isomorphisms
        // ================================

        // Isomorphic nodes
        // --------------------------------

        let number_of_isomorphic_node_groups = isomorphic_node_groups.len() as NodeT;
        let number_of_nodes_involved_in_isomorphic_node_groups = isomorphic_node_groups
            .iter()
            .map(|isomorphic_node_group| isomorphic_node_group.len() as NodeT)
            .sum();
        let number_of_edges_involved_in_isomorphic_node_groups = isomorphic_node_groups
            .iter()
            .map(|isomorphic_node_group| unsafe {
                (self.get_unchecked_node_degree_from_node_id(isomorphic_node_group[0]) as usize
                    * isomorphic_node_group.len()) as EdgeT
            })
            .sum();
        let maximum_number_of_nodes_in_a_isomorphic_node_group = isomorphic_node_groups
            .iter()
            .map(|isomorphic_node_group| isomorphic_node_group.len() as NodeT)
            .max()
            .unwrap_or(0);
        let maximum_number_of_edges_in_a_isomorphic_node_group = isomorphic_node_groups
            .iter()
            .map(|isomorphic_node_group| unsafe {
                (self.get_unchecked_node_degree_from_node_id(isomorphic_node_group[0]) as usize
                    * isomorphic_node_group.len()) as EdgeT
            })
            .max()
            .unwrap_or(0);
        let isomorphic_node_groups_description = self.get_report_of_oddity(
            "h4",
            "Isomorphic node group",
            "Isomorphic node groups",
            concat!(
                "Isomorphic node groups are nodes with exactly the same ",
                "neighbours, node types, edge types and weights (if present in the graph). ",
                "Nodes in such groups are topologically indistinguishable, ",
                "that is swapping their ID would not change the graph topology."
            ),
            number_of_isomorphic_node_groups,
            number_of_nodes_involved_in_isomorphic_node_groups,
            number_of_edges_involved_in_isomorphic_node_groups,
            maximum_number_of_nodes_in_a_isomorphic_node_group,
            maximum_number_of_edges_in_a_isomorphic_node_group,
            true,
            Some(15),
            Some(3),
            isomorphic_node_groups
                .into_iter()
                .map(|isomorphic_node_group| {
                    format!(
                        concat!("<p>Group with {} nodes ({}): {}.</p>",),
                        to_human_readable_high_integer(isomorphic_node_group.len() as usize),
                        unsafe {
                            self.get_unchecked_succinct_node_attributes_description(
                                isomorphic_node_group[0],
                                0,
                                true,
                            )
                        },
                        unsafe {
                            get_unchecked_formatted_list(
                                &isomorphic_node_group
                                    .into_iter()
                                    .map(|node_id| {
                                        get_node_source_html_url_from_node_name(
                                            &self.get_unchecked_node_name_from_node_id(node_id),
                                        )
                                    })
                                    .collect::<Vec<String>>(),
                                Some(5),
                            )
                        }
                    )
                }),
        );

        // Isomorphic edges
        // --------------------------------

        // For large graphs, we do not want to compute isomorphic edges, as it is too expensive.
        let isomorphic_edge_groups_description = if self.get_number_of_edges() < 200_000 {
            let number_of_isomorphic_edge_groups = isomorphic_edge_groups.len() as NodeT;
            let number_of_nodes_involved_in_isomorphic_edge_groups = isomorphic_edge_groups
                .iter()
                .map(|isomorphic_edge_group| 2 * isomorphic_edge_group.len() as NodeT)
                .sum::<NodeT>();
            let number_of_edges_involved_in_isomorphic_edge_groups = isomorphic_edge_groups
                .iter()
                .map(|isomorphic_edge_group| unsafe {
                    ((self.get_unchecked_node_degree_from_node_id(isomorphic_edge_group[0][0])
                        as usize
                        + self.get_unchecked_node_degree_from_node_id(isomorphic_edge_group[0][1])
                            as usize)
                        * isomorphic_edge_group.len()) as EdgeT
                })
                .sum();
            let maximum_number_of_edges_in_a_isomorphic_edge_group = isomorphic_edge_groups
                .iter()
                .map(|isomorphic_edge_group| isomorphic_edge_group.len() as NodeT)
                .max()
                .unwrap_or(0);
            let maximum_number_of_edges_connected_to_a_isomorphic_edge_group = isomorphic_edge_groups
                .iter()
                .map(|isomorphic_edge_group| unsafe {
                    ((self.get_unchecked_node_degree_from_node_id(isomorphic_edge_group[0][0])
                        as usize
                        + self.get_unchecked_node_degree_from_node_id(isomorphic_edge_group[0][1])
                            as usize)
                        * isomorphic_edge_group.len()) as EdgeT
                })
                .max()
                .unwrap_or(0);
            let isomorphic_edge_groups_description = self.get_report_of_oddity(
                "h4",
                "Isomorphic edge group",
                "Isomorphic edge groups",
                concat!(
                    "Isomorphic edge groups are edges with exactly the same ",
                    "neighbours, node types, edge types and weights (if present in the graph). ",
                    "Edges in such groups are topologically indistinguishable, ",
                    "that is swapping their ID would not change the graph topology."
                ),
                number_of_isomorphic_edge_groups,
                number_of_nodes_involved_in_isomorphic_edge_groups,
                number_of_edges_involved_in_isomorphic_edge_groups,
                maximum_number_of_edges_in_a_isomorphic_edge_group,
                maximum_number_of_edges_connected_to_a_isomorphic_edge_group,
                true,
                Some(15),
                Some(3),
                isomorphic_edge_groups
                    .into_iter()
                    .map(|isomorphic_edge_group| {
                        format!(
                            concat!("<p>Group with {number_of_elements} edges (source {src_attribute}, destination {dst_attribute}): {elements}.</p>",),
                            number_of_elements=to_human_readable_high_integer(isomorphic_edge_group.len() as usize),
                            src_attribute=unsafe {
                                self.get_unchecked_succinct_node_attributes_description(
                                    isomorphic_edge_group[0][0],
                                    0,
                                    true,
                                )
                            },
                            dst_attribute=unsafe {
                                self.get_unchecked_succinct_node_attributes_description(
                                    isomorphic_edge_group[0][1],
                                    0,
                                    true,
                                )
                            },
                            elements=unsafe {
                                get_unchecked_formatted_list(
                                    &isomorphic_edge_group
                                        .into_iter()
                                        .map(|[src, dst]| {
                                            format!(
                                                "({} -> {})",
                                                get_node_source_html_url_from_node_name(
                                                    &self.get_unchecked_node_name_from_node_id(src),
                                                ),
                                                get_node_source_html_url_from_node_name(
                                                    &self.get_unchecked_node_name_from_node_id(dst),
                                                )
                                            )
                                            
                                        })
                                        .collect::<Vec<String>>(),
                                    Some(5),
                                )
                            }
                        )
                    }),
            );
            isomorphic_edge_groups_description
        } else {
            "".to_string()
        };

        // ================================
        // Trees and tree-like oddities
        // ================================

        let tree_like_oddities_description = if tree_like_oddities.is_empty() {
            "".to_string()
        } else {
            let mut tendrils: Vec<DendriticTree> = Vec::new();
            let mut trees: Vec<DendriticTree> = Vec::new();
            let mut dendritic_trees: Vec<DendriticTree> = Vec::new();
            let mut stars: Vec<DendriticTree> = Vec::new();
            let mut tendril_stars: Vec<DendriticTree> = Vec::new();
            let mut dendritic_stars: Vec<DendriticTree> = Vec::new();
            let mut dendritic_tendril_stars: Vec<DendriticTree> = Vec::new();
            let mut free_floating_chains: Vec<DendriticTree> = Vec::new();

            tree_like_oddities.into_iter().for_each(|tree_like_oddity| {
                if tree_like_oddity.is_tree() {
                    trees.push(tree_like_oddity);
                } else if tree_like_oddity.is_star() {
                    stars.push(tree_like_oddity);
                } else if tree_like_oddity.is_tendril() {
                    tendrils.push(tree_like_oddity);
                } else if tree_like_oddity.is_free_floating_chain() {
                    free_floating_chains.push(tree_like_oddity);
                } else if tree_like_oddity.is_dendritic_tree() {
                    dendritic_trees.push(tree_like_oddity);
                } else if tree_like_oddity.is_dendritic_star() {
                    dendritic_stars.push(tree_like_oddity);
                } else if tree_like_oddity.is_dendritic_tendril_star() {
                    dendritic_tendril_stars.push(tree_like_oddity);
                } else if tree_like_oddity.is_tendril_star() {
                    tendril_stars.push(tree_like_oddity);
                } else {
                    unreachable!(
                        "The cases for the different dendritic trees should be fully described."
                    );
                }
            });

            format!(
                concat!(
                    "{trees_description}",
                    "{dendritic_trees_description}",
                    "{stars_description}",
                    "{tendril_stars_description}",
                    "{dendritic_stars_description}",
                    "{dendritic_tendril_stars_description}",
                    "{free_floating_chains_description}",
                    "{tendrils_description}",
                ),
                trees_description=self.get_report_of_specific_tree_like_oddities(
                    trees,
                    "Tree",
                    "Trees",
                    concat!(
                        "A tree is a connected component with <code>n</code> nodes and <code>n-1</code> edges."
                    )
                ),
                dendritic_trees_description=self.get_report_of_specific_tree_like_oddities(
                    dendritic_trees,
                    "Dendritic tree",
                    "Dendritic trees",
                    concat!(
                        "A dendritic tree is a tree-like structure starting from a root node ",
                        "that is part of another strongly connected component."
                    )
                ),
                stars_description=self.get_report_of_specific_tree_like_oddities(
                    stars,
                    "Star",
                    "Stars",
                    concat!(
                        "A star is a tree with a maximal depth of one, where nodes ",
                        "with maximal unique degree one are connected to a central ",
                        "root node with a high degree."
                    )
                ),
                tendril_stars_description=self.get_report_of_specific_tree_like_oddities(
                    tendril_stars,
                    "Tendril star",
                    "Tendril stars",
                    concat!(
                        "A tendril star is a tree with a depth greater than one, ",
                        "where the arms of the star are tendrils."
                    )
                ),
                dendritic_stars_description=self.get_report_of_specific_tree_like_oddities(
                    dendritic_stars,
                    "Dendritic star",
                    "Dendritic stars",
                    concat!(
                        "A dendritic star is a dendritic tree with a maximal depth of one, where nodes ",
                        "with maximal unique degree one are connected to a central ",
                        "root node with high degree and inside a strongly connected component."
                    )
                ),
                dendritic_tendril_stars_description=self.get_report_of_specific_tree_like_oddities(
                    dendritic_tendril_stars,
                    "Dendritic tendril star",
                    "Dendritic tendril stars",
                    concat!(
                        "A dendritic tendril star is a dendritic tree with a depth greater than one, ",
                        "where the arms of the star are tendrils."
                    )
                ),
                free_floating_chains_description=self.get_report_of_specific_tree_like_oddities(
                    free_floating_chains,
                    "Free-floating chain",
                    "Free-floating chains",
                    concat!(
                        "A free-floating chain is a tree with maximal degree two."
                    )
                ),
                tendrils_description=self.get_report_of_specific_tree_like_oddities(
                    tendrils,
                    "Tendril",
                    "Tendrils",
                    concat!(
                        "A tendril is a path starting from a node of degree one, ",
                        "connected to a strongly connected component."
                    )
                ),
            )
        };

        Ok(Some(format!(
            concat!(
                "<h3>Topological Oddities</h3>",
                "<p>",
                "A topological oddity is a set of nodes in the graph that <i>may be derived</i> by ",
                "an error during the generation of the edge list of the graph and, depending ",
                "on the task, could bias the results of topology-based models. ",
                "{directed_graph_note}",
                "In the following paragraph, we will describe the detected topological oddities.",
                "</p>",
                "{circles_description}",
                "{singleton_nodes_description}",
                "{singleton_nodes_with_selfloops_description}",
                "{trap_nodes_description}",
                "{trap_nodes_with_selfloop_description}",
                "{chains_description}",
                "{node_tuples_description}",
                "{isomorphic_node_groups_description}",
                "{isomorphic_edge_groups_description}",
                "{tree_like_oddities_description}",
            ),
            circles_description = circles_description,
            directed_graph_note= if self.is_directed(){
                "Note that in a directed graph we only support the detection of isomorphic nodes and edges. "
            } else {
                ""
            },
            singleton_nodes_description = singleton_nodes_description,
            singleton_nodes_with_selfloops_description = singleton_nodes_with_selfloops_description,
            trap_nodes_description = trap_nodes_description,
            trap_nodes_with_selfloop_description = trap_nodes_with_selfloop_description,
            chains_description = chains_description,
            node_tuples_description = node_tuples_description,
            isomorphic_node_groups_description = isomorphic_node_groups_description,
            isomorphic_edge_groups_description = isomorphic_edge_groups_description,
            tree_like_oddities_description = tree_like_oddities_description,
        )))
    }

    /// Returns report on the graph edge weights
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs that do not contain
    /// edge weights.
    ///
    /// TODO! Add formatting for cases with constant weights.
    /// TODO! Add formatting for cases with negative edge weights.
    unsafe fn get_edge_weights_report(&self) -> String {
        format!(
            concat!(
                "<h3>Weights</h3>",
                "<p>",
                "The minimum edge weight is {minimum_edge_weight}, the maximum edge weight is {maximum_edge_weight} and the total edge weight is {total_edge_weight}. ",
                "The RAM requirement for the edge weights data structure is {ram_edge_weights}.",
                "</p>",
            ),
            minimum_edge_weight= self.get_mininum_edge_weight().clone().unwrap(),
            maximum_edge_weight= self.get_maximum_edge_weight().clone().unwrap(),
            total_edge_weight=self.get_total_edge_weights().clone().unwrap(),
            ram_edge_weights=self.get_edge_weights_total_memory_requirements_human_readable()
        )
    }

    /// Returns report on the singleton node types of the graph.
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs without
    /// singleton node types.
    unsafe fn get_singleton_node_types_report(&self) -> String {
        format!(
            concat!(
                "<h4>Singleton node types</h4>",
                "<p>Singleton node types are node types that are assigned ",
                "exclusively to a single node, making the node type ",
                "relatively meaningless, as it adds no more information ",
                "than the node name itself. ",
                "The graph contains {singleton_number_of_node_types}.</p>"
            ),
            singleton_number_of_node_types = match self.get_number_of_singleton_node_types().unwrap() {
                1 => {
                    let node_type_name = self
                        .iter_singleton_node_type_names()
                        .unwrap()
                        .next()
                        .unwrap();
                    format!(
                        "a singleton node type, which is {} (node {})",
                        get_node_type_source_html_url_from_node_type_name(&node_type_name),
                        self.get_unchecked_succinct_node_description(
                            self.get_node_ids_from_node_type_name(&node_type_name)
                                .unwrap()[0],
                            0,
                            true
                        )
                    )
                }
                singleton_number_of_node_types => {
                    format!(
                        concat!(
                            "{singleton_number_of_node_types} singleton node types, which are ",
                            "{singleton_node_types_list}",
                            "{additional_singleton_node_types}"
                        ),
                        singleton_number_of_node_types =
                            to_human_readable_high_integer(singleton_number_of_node_types as usize),
                        singleton_node_types_list = get_unchecked_formatted_list(
                            self.iter_singleton_node_type_names()
                                .unwrap()
                                .take(10)
                                .map(|node_type_name| {
                                    format!(
                                        "{} ({})",
                                        get_node_type_source_html_url_from_node_type_name(
                                            node_type_name.as_ref(),
                                        ),
                                        self.get_unchecked_succinct_node_description(
                                            self.get_node_ids_from_node_type_name(&node_type_name)
                                                .unwrap()[0],
                                            0,
                                            true
                                        )
                                    )
                                })
                                .collect::<Vec<_>>()
                                .as_ref(),
                            None
                        ),
                        additional_singleton_node_types = if singleton_number_of_node_types > 10 {
                            format!(
                                ", plus other {singleton_number_of_node_types} singleton node types",
                                singleton_number_of_node_types = to_human_readable_high_integer(
                                    singleton_number_of_node_types as usize - 10
                                )
                            )
                        } else {
                            "".to_string()
                        }
                    )
                }
            }
        )
    }

    /// Returns report on the isomorphic node types of the graph.
    unsafe fn get_isomorphic_node_types_report(&self) -> String {
        let threshold = 50_000;
        let use_approximation = self.get_number_of_node_types().unwrap() > threshold;
        let mut isomorphic_node_types = if use_approximation {
            self.get_approximated_isomorphic_node_type_ids_groups()
                .unwrap()
        } else {
            self.get_isomorphic_node_type_ids_groups().unwrap()
        };
        isomorphic_node_types.par_sort_unstable_by(|a, b| {
            (self.get_unchecked_number_of_nodes_from_node_type_id(b[0]) as usize * b.len()).cmp(
                &(self.get_unchecked_number_of_nodes_from_node_type_id(a[0]) as usize * a.len()),
            )
        });
        if isomorphic_node_types.is_empty() {
            "".to_string()
        } else {
            let isomorphic_number_of_node_types = isomorphic_node_types.len();
            format!(
                concat!(
                    "<h4>{oddity_name}</h4>",
                    "<p>",
                    "{oddity_name} groups are node types describing ",
                    "exactly the same set of nodes.{approximation_note} The presence of such duplicated ",
                    "node types suggests a potential modelling error in the pipeline ",
                    "that has produced this graph. {isomorphic_number_of_node_types} isomorphic node types groups ",
                    "were detected in this graph.",
                    "</p>",
                    "<ol>",
                    "{isomorphic_node_types_description}",
                    "</ol>",
                    "{additional_isomorphic_node_types}"
                ),
                oddity_name= if use_approximation {
                    "Approximated isomorphic node types"
                } else {
                    "Isomorphic node types"
                },
                approximation_note = if use_approximation {
                    format!(
                        concat!(
                            " Since the graph has a high number of node types (> {}) ",
                            "we use an approximated version of the node types isomorphisms ",
                            "detection. Note that hash collisions and false positives are possible. ",
                            "It is possible to execute the exact computation with ",
                            "the <code>get_isomorphic_node_type_ids_groups</code> method."
                        ),
                        to_human_readable_high_integer(threshold as usize)
                    )
                } else {
                    "".to_string()
                },
                isomorphic_node_types_description = isomorphic_node_types.into_iter().take(10).map(|isomorphic_node_type_group| {
                    format!(
                        concat!(
                            "<li><p>Isomorphic node type group containing {} node types ({}), which are: {}.</p></li>",
                        ),
                        to_human_readable_high_integer(isomorphic_node_type_group.len() as usize),
                        self.get_unchecked_succinct_node_type_attributes_description(
                            isomorphic_node_type_group[0],
                            None,
                            None
                        ),
                         {
                            get_unchecked_formatted_list(
                                &isomorphic_node_type_group
                                    .into_iter()
                                    .map(|node_type_id| {
                                        get_node_type_source_html_url_from_node_type_name(&self
                                            .get_node_type_name_from_node_type_id(node_type_id).unwrap())
                                    })
                                    .collect::<Vec<String>>(),
                                Some(5),
                            )
                        }
                    )
                }).join("\n"),
                isomorphic_number_of_node_types = to_human_readable_high_integer(isomorphic_number_of_node_types),
                additional_isomorphic_node_types =
                            if isomorphic_number_of_node_types > 10 {
                                format!(
                                "<p>And other {isomorphic_number_of_node_types} isomorphic node types.</p>",
                                isomorphic_number_of_node_types = to_human_readable_high_integer(
                                    isomorphic_number_of_node_types as usize - 10
                                )
                            )
                            } else {
                                "".to_string()
                            }
            )
        }
    }

    /// Returns report on the isomorphic edge types of the graph.
    unsafe fn get_isomorphic_edge_types_report(&self) -> String {
        let mut isomorphic_edge_types = self.get_isomorphic_edge_type_ids_groups(Some(20)).unwrap();
        isomorphic_edge_types.par_sort_unstable_by(|a, b| {
            (self.get_unchecked_number_of_edges_from_edge_type_id(b[0]) as usize * b.len()).cmp(
                &(self.get_unchecked_number_of_edges_from_edge_type_id(a[0]) as usize * a.len()),
            )
        });
        if isomorphic_edge_types.is_empty() {
            "".to_string()
        } else {
            let isomorphic_number_of_edge_types = isomorphic_edge_types.len();
            format!(
                concat!(
                    "<h4>Isomorphic edge types</h4>",
                    "<p>",
                    "Isomorphic edge types groups are edge types describing ",
                    "exactly the same set of edges. The presence of such duplicated ",
                    "edge types suggests a potential modelling error in the pipeline ",
                    "that has produced this graph. {isomorphic_number_of_edge_types} isomorphic edge types groups ",
                    "were detected in this graph. ",
                    "We currently compute and display the isomorphic edge types with at least 20 edges.",
                    "</p>",
                    "<ol>",
                    "{isomorphic_edge_types_description}",
                    "</ol>",
                    "{additional_isomorphic_edge_types}"
                ),
                isomorphic_edge_types_description = isomorphic_edge_types.into_iter().take(10).map(|isomorphic_edge_type_group| {
                    format!(
                        concat!(
                            "<li><p>Isomorphic edge type group containing {} edge types ({}), which are: {}.</p></li>",
                        ),
                        to_human_readable_high_integer(isomorphic_edge_type_group.len() as usize),
                        self.get_unchecked_succinct_edge_type_attributes_description(isomorphic_edge_type_group[0], None, None),
                        {
                            get_unchecked_formatted_list(
                                &isomorphic_edge_type_group
                                    .into_iter()
                                    .map(|edge_type_id| {
                                        get_edge_type_source_html_url_from_edge_type_name(&self
                                            .get_unchecked_edge_type_name_from_edge_type_id(Some(edge_type_id)).unwrap())
                                    })
                                    .collect::<Vec<String>>(),
                                Some(5),
                            )
                        }
                    )
                }).join("\n"),
                isomorphic_number_of_edge_types = to_human_readable_high_integer(isomorphic_number_of_edge_types),
                additional_isomorphic_edge_types =
                            if isomorphic_number_of_edge_types > 10 {
                                format!(
                                "<p>And other {isomorphic_number_of_edge_types} isomorphic edge types.</p>",
                                isomorphic_number_of_edge_types = to_human_readable_high_integer(
                                    isomorphic_number_of_edge_types as usize - 10
                                )
                            )
                            } else {
                                "".to_string()
                            }
            )
        }
    }

    /// Returns report on the homogeneous node types of the graph.
    unsafe fn get_homogeneous_node_types_report(&self) -> String {
        format!(
            concat!(
                "<h4>Homogeneous node types</h4>",
                "<p>Homogeneous node types are node types that are assigned ",
                "to all the nodes in the graph, making the node type ",
                "relatively meaningless, as it adds no more information ",
                "than the fact that the node is in the graph. ",
                "The graph contains {homogeneous_number_of_node_types}.</p>"
            ),
            homogeneous_number_of_node_types = match self
                .get_number_of_homogeneous_node_types()
                .unwrap()
            {
                1 => format!(
                    "a homogeneous node type, which is {}",
                    get_node_type_source_html_url_from_node_type_name(
                        self.iter_homogeneous_node_type_names()
                            .unwrap()
                            .next()
                            .unwrap()
                            .as_ref()
                    )
                ),
                homogeneous_number_of_node_types => {
                    format!(
                        concat!(
                            "{homogeneous_number_of_node_types} homogeneous node types, which are ",
                            "{homogeneous_node_types_list}",
                            "{additional_homogeneous_nodes_with_selfloop}"
                        ),
                        homogeneous_number_of_node_types =
                            to_human_readable_high_integer(homogeneous_number_of_node_types as usize),
                        homogeneous_node_types_list = get_unchecked_formatted_list(
                            self.iter_homogeneous_node_type_names()
                                .unwrap()
                                .take(10)
                                .map(|node_type_name| {
                                    get_node_type_source_html_url_from_node_type_name(
                                        node_type_name.as_ref(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .as_ref(),
                            None
                        ),
                        additional_homogeneous_nodes_with_selfloop =
                            if homogeneous_number_of_node_types > 10 {
                                format!(
                                ", plus other {homogeneous_number_of_node_types} homogeneous node types",
                                homogeneous_number_of_node_types = to_human_readable_high_integer(
                                    homogeneous_number_of_node_types as usize - 10
                                )
                            )
                            } else {
                                "".to_string()
                            }
                    )
                }
            }
        )
    }

    /// Returns report on the unknown types of the graph.
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs without
    /// unknown types.
    unsafe fn get_unknown_node_types_report(&self) -> String {
        format!(
            concat!(
                "<h4>Unknown node types</h4>",
                "<p>Nodes with unknown node types are nodes with a ",
                "node type that was not provided during the creation of ",
                "the graph, which may be desired as the output of a ",
                "node-label holdout. ",
                "The graph contains {unknown_number_of_node_types}, making up {unknown_node_types_percentage:.2} of the nodes.</p>"
            ),
            unknown_node_types_percentage = self.get_unknown_node_types_rate().unwrap() * 100.0,
            unknown_number_of_node_types = match self.get_number_of_unknown_node_types().unwrap() {
                1 => format!(
                    "a node with unknown node type, which is {}",
                    self.get_unchecked_succinct_node_description(
                        self.iter_node_ids_with_unknown_node_types()
                            .unwrap()
                            .next()
                            .unwrap(), 0, false
                    )
                ),
                unknown_number_of_node_types => {
                    format!(
                        concat!(
                            "{unknown_number_of_node_types} nodes with unknown node type, which are ",
                            "{unknown_node_types_list}",
                            "{additional_unknown_nodes}"
                        ),
                        unknown_number_of_node_types = to_human_readable_high_integer(unknown_number_of_node_types as usize),
                        unknown_node_types_list = get_unchecked_formatted_list(
                            self.iter_node_ids_with_unknown_node_types()
                                .unwrap()
                                .take(10)
                                .map(|node_id| {
                                    self.get_unchecked_succinct_node_description(node_id, 0, false)
                                })
                                .collect::<Vec<_>>()
                                .as_ref(),
                                None
                        ),
                        additional_unknown_nodes = if unknown_number_of_node_types > 10 {
                            format!(
                                ", plus other {unknown_number_of_node_types} nodes with unknown node types",
                                unknown_number_of_node_types = to_human_readable_high_integer(unknown_number_of_node_types as usize - 10)
                            )
                        } else {
                            "".to_string()
                        }
                    )
                }
            }
        )
    }

    /// Returns the node types description for the provided count.
    ///
    /// # Arguments
    /// * `count`: HashMap<NodeTypeT, NodeT> - The node type description count.
    ///
    /// # Safety
    /// It is assumed that the provided count makes sense with the current graph instance
    /// or it may lead to panic or undefined behaviours.
    pub(crate) unsafe fn get_unchecked_node_types_description_from_count(
        &self,
        count: HashMap<NodeTypeT, NodeT>,
    ) -> String {
        match count.len() {
            0 => {
                unreachable!("It does not make sense to require the description of an empty count.")
            }
            1 => format!(
                concat!("a single node type, which is {node_type_description}",),
                node_type_description = get_node_type_source_html_url_from_node_type_name(
                    self.get_node_type_name_from_node_type_id(count.into_keys().last().unwrap())
                        .unwrap()
                        .as_ref()
                )
            ),
            number_of_node_types => {
                let mut count = count.into_iter().collect::<Vec<(NodeTypeT, NodeT)>>();
                count.sort_by(|(_, a), (_, b)| b.cmp(a));
                let node_type_descriptions = get_unchecked_formatted_list(
                    count
                        .into_iter()
                        .take(10)
                        .map(|(node_type_id, count)| {
                            self.get_unchecked_succinct_node_type_description(
                                node_type_id,
                                None,
                                Some(count),
                            )
                        })
                        .collect::<Vec<_>>()
                        .as_ref(),
                    None,
                );
                format!(
                    "{number_of_node_types} node types, {top_ten_caveat} {node_type_description}",
                    number_of_node_types = to_human_readable_high_integer(number_of_node_types as usize),
                    top_ten_caveat = if number_of_node_types > 10 {
                        "of which the 10 most common are"
                    } else {
                        "which are"
                    },
                    node_type_description = node_type_descriptions
                )
            }
        }
    }

    /// Returns report on the graph node types.
    ///
    /// # Safety
    /// This method may raise a panic when called on graph instances
    /// without node types.
    ///
    /// TODO! Add paragram handling the corner case where all node types are unknown.
    unsafe fn get_node_types_report(&self) -> String {
        // First we define the list of paragraphs of the report.
        let mut paragraphs = Vec::new();

        paragraphs.push(format!(
            concat!(
                "<h3>Node types</h3>",
                "<p>",
                "The graph has {node_types_description}.{multilabel_node_types} ",
                "The RAM requirement for the node types data structure is {ram_node_types}.",
                "</p>",
            ),
            node_types_description = self.get_unchecked_node_types_description_from_count(
                self.get_node_type_id_counts_hashmap().unwrap()
            ),
            multilabel_node_types = if self.has_multilabel_node_types().unwrap() {
                format!(
                    concat!(
                        " The node types are multi-label, and the node ",
                        "with most node types has {} node types."
                    ),
                    self.get_maximum_multilabel_count().unwrap()
                )
            } else {
                "".to_string()
            },
            ram_node_types = self
                .get_node_types_total_memory_requirements_human_readable()
                .unwrap()
        ));

        // When the graph contains multilabel node types, we build the report
        // relative to the isomorphic node types.
        if self.has_multilabel_node_types().unwrap() {
            paragraphs.push(self.get_isomorphic_node_types_report());
        }

        // When the graph contains singleton node types, we build their report.
        if self.has_singleton_node_types().unwrap() {
            paragraphs.push(self.get_singleton_node_types_report());
        }

        // When the graph contains homogeneous node types, we build their report.
        if self.has_homogeneous_node_types().unwrap() {
            paragraphs.push(self.get_homogeneous_node_types_report());
        }

        // When the graph contains unknown node types, we build their report.
        if self.has_unknown_node_types().unwrap() {
            paragraphs.push(self.get_unknown_node_types_report());
        }

        paragraphs.join("")
    }

    /// Returns report on the singleton edge types of the graph.
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs without
    /// singleton edge types.
    unsafe fn get_singleton_edge_types_report(&self) -> String {
        format!(
            concat!(
                "<h4>Singleton edge types</h4>",
                "<p>Singleton edge types are edge types that are assigned ",
                "exclusively to a single edge, making the edge type ",
                "relatively meaningless, as it adds no more information ",
                "than the name of edge itself. ",
                "The graph contains {singleton_number_of_edge_types}</p>"
            ),
            singleton_number_of_edge_types = match self.get_number_of_singleton_edge_types().unwrap() {
                1 => format!(
                    "a edge with singleton edge type, which is {}.",
                    get_edge_type_source_html_url_from_edge_type_name(
                        self.iter_singleton_edge_type_names()
                            .unwrap()
                            .next()
                            .unwrap()
                            .as_ref()
                    )
                ),
                singleton_number_of_edge_types => {
                    format!(
                        concat!(
                            "{singleton_number_of_edge_types} edges with singleton edge types, which are ",
                            "{singleton_edge_types_list}",
                            "{additional_edgges_with_singleton_edge_types}. "
                        ),
                        singleton_number_of_edge_types = to_human_readable_high_integer(singleton_number_of_edge_types as usize),
                        singleton_edge_types_list = get_unchecked_formatted_list(
                            self.iter_singleton_edge_type_names()
                                .unwrap()
                                .take(10)
                                .map(|edge_type_name| {
                                    get_edge_type_source_html_url_from_edge_type_name(
                                        edge_type_name.as_ref(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .as_ref(),
                                None
                        ),
                        additional_edgges_with_singleton_edge_types =
                            if singleton_number_of_edge_types > 10 {
                                format!(
                                ", plus other {singleton_number_of_edge_types} edges with singleton edge types",
                                singleton_number_of_edge_types = to_human_readable_high_integer(singleton_number_of_edge_types as usize - 10)
                            )
                            } else {
                                "".to_string()
                            }
                    )
                }
            }
        )
    }

    /// Returns report on the unknown edge types of the graph.
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs without
    /// unknown types.
    unsafe fn get_unknown_edge_types_report(&self) -> String {
        format!(
            concat!(
                "<h4>Unknown edge types</h4>",
                "<p>Edges with unknown edge types are edges with a ",
                "edge type that was not provided during the creation of ",
                "the graph, which may be desired as the output of a ",
                "edge-label holdout. ",
                "The graph contains {unknown_number_of_edge_types}, making up {unknown_edge_types_percentage:.2} of the edges.</p>"
            ),
            unknown_edge_types_percentage = self.get_unknown_edge_types_rate().unwrap() * 100.0,
            unknown_number_of_edge_types = match self.get_number_of_unknown_edge_types().unwrap() {
                1 => format!(
                    "a edge with unknown edge type, which is {}.",
                    self.get_unchecked_succinct_edge_description(
                        self.iter_directed_edge_ids_with_unknown_edge_types()
                            .unwrap()
                            .next()
                            .unwrap()
                    )
                ),
                unknown_types_number => {
                    format!(
                        concat!(
                            "{unknown_types_number} edges with unknown edge type, which are ",
                            "{unknown_edge_types_list}",
                            "{additional_unknown_edges}."
                        ),
                        unknown_types_number = unknown_types_number,
                        unknown_edge_types_list = get_unchecked_formatted_list(
                            self.iter_directed_edge_ids_with_unknown_edge_types()
                                .unwrap()
                                .take(10)
                                .map(|edge_id| {
                                    self.get_unchecked_succinct_edge_description(edge_id)
                                })
                                .collect::<Vec<_>>()
                                .as_ref(),
                                None
                        ),
                        additional_unknown_edges = if unknown_types_number > 10 {
                            format!(
                                ", plus other {unknown_types_number} edges with unknown edge types",
                                unknown_types_number = unknown_types_number - 10
                            )
                        } else {
                            "".to_string()
                        }
                    )
                }
            }
        )
    }

    /// Returns the edge types description for the provided count.
    ///
    /// # Arguments
    /// * `count`: HashMap<EdgeTypeT, edgeT> - The edge type description count.
    ///
    /// # Safety
    /// It is assumed that the provided count makes sense with the current graph instance
    /// or it may lead to panic or undefined behaviours.
    pub(crate) unsafe fn get_unchecked_edge_types_description_from_count(
        &self,
        count: HashMap<EdgeTypeT, EdgeT>,
    ) -> String {
        match count.len() {
            0 => "".to_string(),
            1 => format!(
                concat!("a single edge type, which is {edge_type_description}",),
                edge_type_description = get_edge_type_source_html_url_from_edge_type_name(
                    self.get_edge_type_name_from_edge_type_id(count.into_keys().last().unwrap())
                        .unwrap()
                        .as_ref()
                )
            ),
            number_of_edge_types => {
                let mut edge_type_counts = count.into_iter().collect::<Vec<_>>();
                edge_type_counts.par_sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
                let total_edges = Some(
                    edge_type_counts
                        .iter()
                        .map(|(_, number_of_edges)| *number_of_edges)
                        .sum(),
                );
                let edge_type_descriptions = get_unchecked_formatted_list(
                    edge_type_counts
                        .into_iter()
                        .take(10)
                        .map(|(edge_type_id, count)| {
                            self.get_unchecked_succinct_edge_type_description(
                                edge_type_id,
                                total_edges,
                                Some(count),
                            )
                        })
                        .collect::<Vec<_>>()
                        .as_ref(),
                    None,
                );
                format!(
                    "{number_of_edge_types} edge types, {top_ten_caveat} {edge_type_description}",
                    number_of_edge_types = to_human_readable_high_integer(number_of_edge_types as usize),
                    top_ten_caveat = if number_of_edge_types > 10 {
                        "of which the 10 most common are"
                    } else {
                        "which are"
                    },
                    edge_type_description = edge_type_descriptions
                )
            }
        }
    }

    /// Returns report on the graph edge types.
    ///
    /// # Safety
    /// This method may raise a panic when called on graph instances
    /// without edge types.
    ///
    /// TODO! Add paragram handling the corner case where all edge types are unknown.
    unsafe fn get_edge_types_report(&self) -> String {
        // First we define the list of paragraphs of the report.
        let mut paragraphs = Vec::new();

        paragraphs.push(format!(
            concat!(
                "<h3>Edge types</h3>",
                "<p>",
                "The graph has {number_of_edge_types}. ",
                "The RAM requirement for the edge types data structure is {ram_edge_types}.",
                "</p>",
            ),
            number_of_edge_types = self.get_unchecked_edge_types_description_from_count(
                self.get_edge_type_id_counts_hashmap().unwrap()
            ),
            ram_edge_types = self
                .get_edge_types_total_memory_requirements_human_readable()
                .unwrap()
        ));

        if self.is_multigraph() {
            paragraphs.push(self.get_isomorphic_edge_types_report());
        }

        // When the graph contains singleton edge types, we build their report.
        if self.has_singleton_edge_types().unwrap() {
            paragraphs.push(self.get_singleton_edge_types_report());
        }

        // When the graph contains unknown edge types, we build their report.
        if self.has_unknown_edge_types().unwrap() {
            paragraphs.push(self.get_unknown_edge_types_report());
        }

        paragraphs.join("")
    }

    /// Return html short textual report of the graph.
    ///
    /// TODO! Add reports on various node metrics
    /// TODO! Add reports on various edge metrics
    /// NOTE! Most of the above TODOs will require first to implement the
    /// support for the fast computation of the inbound edges in a directed
    /// graphs.
    pub fn textual_report(&self) -> String {
        // First of all we create the empty list of report paragraphs
        let mut paragraphs = Vec::new();

        // We add to the report paragrams the one with the brief summary
        paragraphs.push(self.get_textual_report_summary());

        // if the graph has at least an edge.
        if self.has_edges() {
            // We add to the report the unweighted node degree centrality
            paragraphs.push(unsafe { self.get_node_degree_centrality_report() });
        }

        // We add to the report the edge weights report if the graph
        if self.has_edge_weights() {
            paragraphs.push(unsafe { self.get_edge_weights_report() });
        }

        // We add the report on the node types
        // For the time being I am dropping this section of the report when the graph
        // contains exclusively unknown node types.
        if self.has_node_types() && self.has_known_node_types().unwrap() {
            paragraphs.push(unsafe { self.get_node_types_report() });
        }

        // We add the report on the edge types
        // For the time being I am dropping this section of the report when the graph
        // contains exclusively unknown edge types.
        if self.has_edge_types() && self.has_known_edge_types().unwrap() {
            paragraphs.push(unsafe { self.get_edge_types_report() });
        }

        // And the report with oddities, if there are any to report
        if self.has_edges() {
            if let Some(oddity_report) = self.get_report_of_topological_oddities().unwrap() {
                paragraphs.push(oddity_report);
            }
        }

        let style = concat!(
            "<style>",
            ".graph-report li {",
            "margin: 0.5em 0 0.5em 0;",
            "}",
            ".graph-report .paragraph {",
            "text-align: justify;",
            "word-break: break-all;",
            "}",
            ".graph-report .small-columns {",
            "column-count: 4;",
            "column-gap: 2em;",
            "}",
            ".graph-report .medium-columns {",
            "column-count: 3;",
            "column-gap: 2em;",
            "}",
            ".graph-report .large-columns {",
            "column-count: 2;",
            "column-gap: 2em;",
            "}",
            ".graph-report .single-column {}",
            "@media only screen and (max-width: 600px) {",
            ".graph-report .small-columns {",
            "column-count: 1;",
            "}",
            ".graph-report .medium-columns {",
            "column-count: 1;",
            "}",
            ".graph-report .large-columns {",
            "column-count: 1;",
            "}",
            "}",
            "@media only screen and (min-width: 600px) and (max-width: 800px) {",
            ".graph-report .small-columns {",
            "column-count: 2;",
            "}",
            ".graph-report .medium-columns {",
            "column-count: 1;",
            "}",
            ".graph-report .large-columns {",
            "column-count: 1;",
            "}",
            "}",
            "@media only screen and (min-width: 800px) and (max-width: 1024px) {",
            ".graph-report .small-columns {",
            "column-count: 3;",
            "}",
            ".graph-report .medium-columns {",
            "column-count: 2;",
            "}",
            ".graph-report .large-columns {",
            "column-count: 1;",
            "}",
            "}",
            "</style>"
        );
        let mut report = format!(
            "<div class=\"graph-report\">{}{}</div>",
            style,
            paragraphs.join("")
        );
        report = report.replace("<p>", "<p class=\"paragraph\">");
        report = report.replace("<h3>", "<h3 style=\"margin: 1em 0 0 0;\">");
        report = report.replace("<h4>", "<h4 style=\"margin: 1em 0 0 0;\">");
        report = report.replace("<h5>", "<h5 style=\"margin: 1em 0 0 0;\">");
        report
    }
}
