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
        report.insert("nodes_number", self.get_nodes_number().to_string());
        report.insert(
            "singleton_nodes_number",
            self.get_singleton_nodes_number().to_string(),
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
            "directed_edges_number",
            self.get_directed_edges_number().to_string(),
        );
        report.insert("selfloops_number", self.get_selfloops_number().to_string());
        report.insert(
            "singleton_nodes_with_selfloops_number",
            self.get_singleton_nodes_with_selfloops_number().to_string(),
        );
        report.insert("multigraph", self.is_multigraph().to_string());
        report.insert(
            "parallel_edges_number",
            self.get_parallel_edges_number().to_string(),
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
                "node_types_number",
                self.get_node_types_number().unwrap().to_string(),
            );
            report.insert(
                "unknown_node_types_number",
                self.get_unknown_node_types_number().unwrap().to_string(),
            );
            report.insert(
                "known_node_types_number",
                self.get_known_node_types_number().unwrap().to_string(),
            );
        }
        report.insert("has_edge_types", self.has_edge_types().to_string());
        if self.has_edge_types() {
            report.insert(
                "has_singleton_edge_types",
                self.has_singleton_edge_types().unwrap().to_string(),
            );
            report.insert(
                "edge_types_number",
                self.get_edge_types_number().unwrap().to_string(),
            );
            report.insert(
                "unknown_edge_types_number",
                self.get_unknown_edge_types_number().unwrap().to_string(),
            );
            report.insert(
                "known_edge_types_number",
                self.get_known_edge_types_number().unwrap().to_string(),
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
        let overlapping_nodes_number = self
            .iter_node_names_and_node_type_names()
            .filter(|(_, node_name, _, node_type)| {
                other.has_node_name_and_node_type_name(node_name, node_type.clone())
            })
            .count();
        // Get overlapping edges
        let overlapping_edges_number = self
            .par_iter_edge_node_names_and_edge_type_name(self.directed)
            .filter(|(_, _, src_name, _, dst_name, _, edge_type_name)| {
                other.has_edge_from_node_names_and_edge_type_name(
                    src_name,
                    dst_name,
                    edge_type_name.as_deref(),
                )
            })
            .count();
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
                match self.get_singleton_nodes_number() {
                    1 => format!(
                        concat!("The given node {} is the only singleton node of the graph."),
                        node_name
                    ),
                    singleton_nodes_number => {
                        format!(
                            concat!("The given node {} is one of {} singleton nodes."),
                            node_name, singleton_nodes_number
                        )
                    }
                }
            } else if unsafe { self.is_unchecked_singleton_with_selfloops_from_node_id(node_id) } {
                match self.get_singleton_nodes_with_selfloops_number() {
                    1 => format!(
                        concat!(
                        "The given node {} is the only singleton node with selfloops in the graph."
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
                match self.get_trap_nodes_number() {
                    1 => format!(
                        concat!("The given node {} is the only trap node in the graph."),
                        node_name
                    ),
                    trap_nodes_number => {
                        format!(
                            concat!("The given node {} is one of {} trap nodes in the graph."),
                            node_name, trap_nodes_number
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
    ///
    /// # Safety
    /// This method will cause an out of bound if the given node ID does not exist.
    pub(crate) unsafe fn get_unchecked_succinct_node_description(&self, node_id: NodeT) -> String {
        let node_name = self.get_unchecked_node_name_from_node_id(node_id);
        let node_name = get_node_source_html_url_from_node_name(node_name.as_ref());
        let node_type = if self.has_node_types() {
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
                                .as_ref()
                        )
                    )),
                },
                None => Some("unknown node type".to_string()),
            }
        } else {
            None
        };
        let mut node_degree = match self.get_node_degree_from_node_id(node_id) {
            Ok(degree) => {
                if degree == 0 {
                    None
                } else {
                    Some(format!("degree {}", degree))
                }
            }
            Err(_) => None,
        };
        // Update the node degree with also the weighted degree.
        if self.has_edge_weights() {
            node_degree = node_degree.map(|degree_string| {
                format!(
                    "{degree_string}{join_term} weighted degree {weighted_degree:.2}",
                    degree_string = degree_string,
                    // According to the presence of the node type segment
                    // of the description we add the correct join term
                    join_term = if node_type.is_some() { "," } else { " and" },
                    weighted_degree = self.get_unchecked_weighted_node_degree_from_node_id(node_id)
                )
            });
        }

        // If any of the terms was given we build the output description
        let description = if node_degree.is_some() || node_type.is_some() {
            let node_degree_is_some = node_degree.is_some();
            format!(
                " ({node_degree}{join_term}{node_type})",
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

        format!(
            "{node_name}{description}",
            node_name = node_name,
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
            source_node_description = self.get_unchecked_succinct_node_description(self.get_unchecked_source_node_id_from_edge_id(edge_id)),
            destination_node_description = self.get_unchecked_succinct_node_description(self.get_unchecked_destination_node_id_from_edge_id(edge_id)),
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
        let nodes_number = unsafe {
            match self.get_nodes_number() {
                1 => format!(
                    "a single node called {node_name_description}",
                    node_name_description = self.get_unchecked_succinct_node_description(0),
                ),
                nodes_number => format!(
                    "{nodes_number}{heterogeneous_nodes} nodes",
                    nodes_number = nodes_number,
                    heterogeneous_nodes = match self.get_node_types_number() {
                        Ok(n) =>
                            if n == 1 {
                                " homogeneous"
                            } else {
                                " heterogenous"
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
                    "<p>The graph{name} contains {nodes_number} and no edges. ",
                    "If this is unexpected, it may have happened because of a ",
                    "mis-parametrization of a filter method uphill.</p>"
                ),
                name = name.unwrap_or_else(|| "".to_string()),
                nodes_number = nodes_number
            ));

            return report.join("");
        }

        // Otherwise we compute a more comprehensive report of the edges.
        let edges_number = unsafe {
            match self.get_edges_number() {
                1 => format!(
                    "a single {edge_description}",
                    edge_description = self.get_unchecked_succinct_edge_description(0)
                ),
                edges_number => format!(
                    "{edges_number}{heterogeneous_edges} edges",
                    edges_number = edges_number,
                    heterogeneous_edges = match self.get_edge_types_number() {
                        Ok(n) =>
                            if n == 1 {
                                " homogeneous"
                            } else {
                                " heterogenous"
                            },
                        Err(_) => "",
                    },
                ),
            }
        };

        // And put the report summary line togheter.
        report.push(format!(
            concat!(
                "<p>The {directionality}{multigraph} graph{name} has {nodes_number} and {edges_number}.</p>",
                "<h3>RAM requirements</h3>",
                "<p>The RAM requirements for the nodes and edges data structures are {ram_nodes} and {ram_edges} respectively.</p>"
            ),
            directionality = if self.is_directed() {
                "directed"
            } else {
                "undirected"
            },
            multigraph = if self.is_multigraph() {
                " multigraph"
            } else {
                ""
            },
            name = name.unwrap_or_else(|| "".to_string()),
            nodes_number = nodes_number,
            edges_number = edges_number,
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
                "<p>The nodes with highest degree centrality are: {list_of_most_central_nodes}.</p>"
            ),
            minimum_node_degree = self.get_minimum_node_degree().unwrap(),
            maximum_node_degree = self.get_maximum_node_degree().unwrap(),
            mode_node_degree = self.get_node_degrees_mode().unwrap(),
            mean_node_degree = self.get_node_degrees_mean().unwrap(),
            node_degree_median = self.get_node_degrees_median().unwrap(),
            list_of_most_central_nodes = get_unchecked_formatted_list(
                self.get_top_k_central_node_ids(5).unwrap()
                    .into_iter()
                    .filter(|node_id| {
                        self.get_unchecked_node_degree_from_node_id(*node_id) > 0
                    })
                    .map(|node_id| {
                        self.get_unchecked_succinct_node_description(node_id)
                    })
                    .collect::<Vec<_>>()
                    .as_ref()
            )
        )
    }

    /// Returns report on the oddities detected within the graph.
    fn get_report_of_connected_components(&self) -> String {
        let (components_number, minimum_component_size, maximum_component_size) =
            self.get_connected_components_number(None);
        if components_number == 1 {
            return concat!(
                "<h3>Connected components</h3>",
                "<p>The graph is connected, that is, it is composed of a single connected component that includes all nodes and edges.</p>"
            ).to_string();
        }
        format!(
            concat!(
                "<h3>Connected components</h3>",
                "<p>",
                "The graph contains {} connected components, with the largest one containing {} nodes and the smallest one containing {} nodes.",
                "</p>"
            ),
            components_number, maximum_component_size, minimum_component_size,
        )
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
        let mut circles = self.get_circles(None, None)?;
        circles.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        let mut chains = self.get_chains(None, None)?;
        chains.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        let mut stars = self.get_stars(None)?;
        stars.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        // If the graph does not contain any oddity, we do not prepare a report.
        if circles.is_empty() && chains.is_empty() && stars.is_empty() {
            return Ok(None);
        }
        // Create the report for the circles, if there are any.
        let circles_description = if circles.is_empty() {
            "".to_string()
        } else {
            format!(
                concat!(
                    "<h4>Circles</h4>",
                    "<p>",
                    "We have detected {circles_number} circles in the graph, with the largest having {max_circles_size} nodes. ",
                    "The detected circles, sorted by decreasing size, are:",
                    "</p>",
                    "<ol>",
                    "{circles_description}",
                    "</ol>",
                    "{possibly_conclusive_entry}"
                ),
                circles_number = circles.len(),
                max_circles_size = circles.first().unwrap().len(),
                circles_description = circles.iter().take(5).map(|circle| format!("<li>{}</li>", circle.to_string())).join("\n"),
                possibly_conclusive_entry = if circles.len() > 5 {
                    format!(
                        "<p>And other {} circles.</p>",
                        circles.len() -5
                    )
                } else {
                    "".to_string()
                }
            )
        };
        // Create the report for the chains, if there are any.
        let chains_description = if chains.is_empty() {
            "".to_string()
        } else {
            format!(
                concat!(
                    "<h4>Chains</h4>",
                    "<p>",
                    "We have detected {chains_number} chains in the graph, with the largest having {max_chains_size} nodes. ",
                    "The detected chains, sorted by decreasing size, are:",
                    "</p>",
                    "<ol>",
                    "{chains_description}",
                    "</ol>",
                    "{possibly_conclusive_entry}"
                ),
                chains_number = chains.len(),
                max_chains_size = chains.first().unwrap().len(),
                chains_description = chains.iter().take(5).map(|circle| format!("<li>{}</li>", circle.to_string())).join("\n"),
                possibly_conclusive_entry = if chains.len() > 5 {
                    format!(
                        "<p>And other {} chains.</p>",
                        chains.len() -5
                    )
                } else {
                    "".to_string()
                }
            )
        };
        // Create the report for the stars, if there are any.
        let stars_description = if stars.is_empty() {
            "".to_string()
        } else {
            format!(
                concat!(
                    "<h4>stars</h4>",
                    "<p>",
                    "We have detected {stars_number} stars in the graph, with the largest having {max_stars_size} nodes. ",
                    "The detected stars, sorted by decreasing size, are:",
                    "</p>",
                    "<ol>",
                    "{stars_description}",
                    "</ol>",
                    "{possibly_conclusive_entry}"
                ),
                stars_number = stars.len(),
                max_stars_size = stars.first().unwrap().len(),
                stars_description = stars.iter().take(5).map(|circle| format!("<li>{}</li>", circle.to_string())).join("\n"),
                possibly_conclusive_entry = if stars.len() > 5 {
                    format!(
                        "<p>And other {} stars.</p>",
                        stars.len() -5
                    )
                } else {
                    "".to_string()
                }
            )
        };
        Ok(Some(format!(
            concat!(
                "<h3>Topological Oddities</h3>",
                "<p>",
                "A topological oddity is a set of nodes in the graph that may be derived by ",
                "an error during the generation of the edge list of the graph. ",
                "We currently support the detection of <i>Stars</i>, <i>Chains</i>, and <i>Circles</i>. ",
                "In the following paragraph we will describe the detected topological oddities.",
                "</p>",
                "{circles_description}",
                "{chains_description}",
                "{stars_description}",
            ),
            circles_description=circles_description,
            chains_description=chains_description,
            stars_description=stars_description
        )))
    }

    /// Returns report on the singleton nodes of the graph.
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs without
    /// singleton nodes.
    unsafe fn get_singleton_nodes_report(&self) -> String {
        format!(
            concat!(
                "<h4>Singleton nodes</h4>",
                "<p>Singleton nodes are nodes with no edge to other nodes ",
                "nor selfloops. ",
                "The graph contains {singleton_nodes_number}.</p>"
            ),
            singleton_nodes_number = match self.get_singleton_nodes_number() {
                1 => format!(
                    "a singleton node, which is {}",
                    self.get_unchecked_succinct_node_description(
                        self.iter_singleton_node_ids().next().unwrap()
                    )
                ),
                singleton_nodes_number => {
                    format!(
                        concat!(
                            "{singleton_nodes_number} singleton nodes, which are ",
                            "{singleton_nodes_list}",
                            "{additional_singleton_nodes}"
                        ),
                        singleton_nodes_number = singleton_nodes_number,
                        singleton_nodes_list = get_unchecked_formatted_list(
                            self.iter_singleton_node_ids()
                                .take(5)
                                .map(|node_id| {
                                    self.get_unchecked_succinct_node_description(node_id)
                                })
                                .collect::<Vec<_>>()
                                .as_ref()
                        ),
                        additional_singleton_nodes = if singleton_nodes_number > 5 {
                            format!(
                                ", plus other {singleton_nodes_number} singleton nodes",
                                singleton_nodes_number = singleton_nodes_number - 5
                            )
                        } else {
                            ".".to_string()
                        }
                    )
                }
            }
        )
    }

    /// Returns report on the singleton nodes with selfloops of the graph.
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs without
    /// singleton nodes with selfloops.
    unsafe fn get_singleton_nodes_with_selfloops_report(&self) -> String {
        format!(
            concat!(
                "<h4>Singleton nodes with selfloops</h4>",
                "<p>Singleton nodes with selfloops are nodes with no edge to other nodes ",
                "and have exclusively selfloops. ",
                "The graph contains {singleton_nodes_with_selfloops_number}.</p>"
            ),
            singleton_nodes_with_selfloops_number = match self
                .get_singleton_nodes_with_selfloops_number()
            {
                1 => format!(
                    "a singleton node with selfloop, which is {}",
                    self.get_unchecked_succinct_node_description(
                        self.iter_singleton_nodes_with_selfloops_node_ids()
                            .next()
                            .unwrap()
                    )
                ),
                singleton_nodes_with_selfloops_number => {
                    format!(
                        concat!(
                            "{singleton_nodes_with_selfloops_number} singleton nodes with selfloops, which are ",
                            "{singleton_nodes_list}",
                            "{additional_singleton_nodes_with_selfloop}"
                        ),
                        singleton_nodes_with_selfloops_number = singleton_nodes_with_selfloops_number,
                        singleton_nodes_list = get_unchecked_formatted_list(
                            self.iter_singleton_nodes_with_selfloops_node_ids()
                                .take(5)
                                .map(|node_id| {
                                    self.get_unchecked_succinct_node_description(node_id)
                                })
                                .collect::<Vec<_>>()
                                .as_ref()
                        ),
                        additional_singleton_nodes_with_selfloop = if singleton_nodes_with_selfloops_number > 5 {
                            format!(
                                ", plus other {singleton_nodes_with_selfloops_number} singleton nodes with selfloops",
                                singleton_nodes_with_selfloops_number = singleton_nodes_with_selfloops_number - 5
                            )
                        } else {
                            "".to_string()
                        }
                    )
                }
            }
        )
    }

    /// Returns report on the disconnected nodes.
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs without
    /// disconnected nodes.
    unsafe fn get_disconnected_nodes_report(&self) -> String {
        // First we create the empty list of paragraphs of the report
        let mut paragraphs = Vec::new();

        paragraphs.push(format!(
            concat!(
                "<h3>Disconnected nodes</h3>",
                "<p>Disconnected nodes are nodes that are not connected ",
                "to any other node.",
                "The graph contains {disconnected_nodes_number} disconnected nodes.</p>"
            ),
            disconnected_nodes_number = self.get_disconnected_nodes_number()
        ));

        if self.has_singleton_nodes() {
            paragraphs.push(self.get_singleton_nodes_report());
        }

        if self.has_singleton_nodes_with_selfloops() {
            paragraphs.push(self.get_singleton_nodes_with_selfloops_report());
        }

        paragraphs.join("")
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
                "<p>The minimum edge weight is {minimum_edge_weight}, the maximum edge weight is {maximum_edge_weight} and the total edge weight is {total_edge_weight}.</p>",
                "<h4>Weighted degree centrality</h4>",
                "<p>The minimum node degree is {weighted_minimum_node_degree:.2}, the maximum node degree is {weighted_maximum_node_degree:.2}, ",
                "the mean degree is {weighted_mean_node_degree:.2} and the node degree median is {weighted_node_degree_median:2}.</p>",
                "<p>The nodes with highest degree centrality are: {weighted_list_of_most_central_nodes}.</p>",
                "<h4>RAM requirements</h4>",
                "<p>The RAM requirements for the edge weights data structure is {ram_edge_weights}.</p>"
            ),
            minimum_edge_weight= self.get_mininum_edge_weight().clone().unwrap(),
            maximum_edge_weight= self.get_maximum_edge_weight().clone().unwrap(),
            total_edge_weight=self.get_total_edge_weights().clone().unwrap(),
            weighted_minimum_node_degree = self.get_weighted_minimum_node_degree().clone().unwrap(),
            weighted_maximum_node_degree = self.get_weighted_maximum_node_degree().clone().unwrap(),
            weighted_mean_node_degree = self.get_weighted_node_degrees_mean().unwrap(),
            weighted_node_degree_median = self.get_weighted_node_degrees_median().unwrap(),
            weighted_list_of_most_central_nodes = get_unchecked_formatted_list(
                self.get_weighted_top_k_central_node_ids(5).unwrap()
                    .into_iter()
                    .map(|node_id| {
                        self.get_unchecked_succinct_node_description(node_id)
                    })
                    .collect::<Vec<_>>()
                    .as_ref()
            ),
            ram_edge_weights=self.get_edge_weights_total_memory_requirements_human_readable()
        )
    }

    /// Returns report on the singleton node types of the graph.
    ///
    /// # Safety
    /// This method may cause a panic when called on graphs without
    /// singleton node types.
    unsafe fn get_singleton_nodes_types_report(&self) -> String {
        format!(
            concat!(
                "<h4>Singleton node types</h4>",
                "<p>Singleton node types are node types that are assigned ",
                "exclusively to a single node, making the node type ",
                "relatively meaningless, as it adds no more information ",
                "then the name of node itself. ",
                "The graph contains {singleton_nodes_types_number}.</p>"
            ),
            singleton_nodes_types_number = match self.get_singleton_node_types_number().unwrap() {
                1 => format!(
                    "a singleton node type, which is {}",
                    get_node_type_source_html_url_from_node_type_name(
                        self.iter_singleton_node_type_names()
                            .unwrap()
                            .next()
                            .unwrap()
                            .as_ref()
                    )
                ),
                singleton_nodes_types_number => {
                    format!(
                        concat!(
                            "{singleton_nodes_types_number} singleton node types, which are ",
                            "{singleton_node_types_list}",
                            "{additional_singleton_nodes_with_selfloop}"
                        ),
                        singleton_nodes_types_number = singleton_nodes_types_number,
                        singleton_node_types_list = get_unchecked_formatted_list(
                            self.iter_singleton_node_type_names()
                                .unwrap()
                                .take(5)
                                .map(|node_type_name| {
                                    get_node_type_source_html_url_from_node_type_name(
                                        node_type_name.as_ref(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .as_ref()
                        ),
                        additional_singleton_nodes_with_selfloop =
                            if singleton_nodes_types_number > 5 {
                                format!(
                                ", plus other {singleton_nodes_types_number} singleton node types",
                                singleton_nodes_types_number = singleton_nodes_types_number - 5
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
                "The graph contains {unknown_node_types_number}, making up {unknown_node_types_percentage:.2} of the nodes.</p>"
            ),
            unknown_node_types_percentage = self.get_unknown_node_types_rate().unwrap() * 100.0,
            unknown_node_types_number = match self.get_unknown_node_types_number().unwrap() {
                1 => format!(
                    "a node with unknown node type, which is {}",
                    self.get_unchecked_succinct_node_description(
                        self.iter_node_ids_with_unknown_node_types()
                            .unwrap()
                            .next()
                            .unwrap()
                    )
                ),
                unknown_node_types_number => {
                    format!(
                        concat!(
                            "{unknown_node_types_number} nodes with unknown node type, which are ",
                            "{unknown_node_types_list}",
                            "{additional_unknown_nodes}"
                        ),
                        unknown_node_types_number = unknown_node_types_number,
                        unknown_node_types_list = get_unchecked_formatted_list(
                            self.iter_node_ids_with_unknown_node_types()
                                .unwrap()
                                .take(5)
                                .map(|node_id| {
                                    self.get_unchecked_succinct_node_description(node_id)
                                })
                                .collect::<Vec<_>>()
                                .as_ref()
                        ),
                        additional_unknown_nodes = if unknown_node_types_number > 5 {
                            format!(
                                ", plus other {unknown_node_types_number} nodes with unknown node types",
                                unknown_node_types_number = unknown_node_types_number - 5
                            )
                        } else {
                            "".to_string()
                        }
                    )
                }
            }
        )
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
                "<p>The graph has {node_types_number}.</p>",
                "<h4>RAM requirements</h4>",
                "<p>The RAM requirements for the node types data structure is {ram_node_types}.</p>"
            ),
            node_types_number = match self.get_node_types_number().unwrap() {
                1 => format!(
                    concat!(
                        "a single node type, which is {node_type_description}. ",
                        "Note that this means that all nodes have the same ",
                        "node type, that is, all nodes are homogeneous.",
                    ),
                    node_type_description = get_node_type_source_html_url_from_node_type_name(
                        self.get_node_type_name_from_node_type_id(0)
                            .unwrap()
                            .as_ref()
                    )
                ),
                node_types_number => {
                    let mut node_type_counts = self
                        .get_node_type_names_counts_hashmap()
                        .unwrap()
                        .into_iter()
                        .collect::<Vec<_>>();
                    node_type_counts.sort_by(|(_, a), (_, b)| b.cmp(a));
                    let node_type_descriptions = get_unchecked_formatted_list(
                        node_type_counts
                            .into_iter()
                            .take(5)
                            .map(|(node_type_name, count)| {
                                format!(
                                    "{html_url} ({count} nodes, {percentage:.2}%)",
                                    html_url = get_node_type_source_html_url_from_node_type_name(
                                        node_type_name.as_ref()
                                    ),
                                    count = count,
                                    percentage =
                                        (count as f64 / self.get_nodes_number() as f64) * 100.0
                                )
                            })
                            .collect::<Vec<_>>()
                            .as_ref(),
                    );
                    format!(
                        "{node_types_number} node types, {top_five_caveat} {node_type_description}",
                        node_types_number = node_types_number,
                        top_five_caveat = if node_types_number > 5 {
                            "of which the 5 most common are"
                        } else {
                            "which are"
                        },
                        node_type_description = node_type_descriptions
                    )
                }
            },
            ram_node_types = self
                .get_node_types_total_memory_requirements_human_readable()
                .unwrap()
        ));

        // When the graph contains singleton node types, we build their report.
        if self.has_singleton_node_types().unwrap() {
            paragraphs.push(self.get_singleton_nodes_types_report());
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
    unsafe fn get_singleton_edges_types_report(&self) -> String {
        format!(
            concat!(
                "<h4>Singleton edge types</h4>",
                "<p>Singleton edge types are edge types that are assigned ",
                "exclusively to a single edge, making the edge type ",
                "relatively meaningless, as it adds no more information ",
                "then the name of edge itself. ",
                "The graph contains {singleton_edges_types_number}</p>"
            ),
            singleton_edges_types_number = match self.get_singleton_edge_types_number().unwrap() {
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
                singleton_edges_types_number => {
                    format!(
                        concat!(
                            "{singleton_edges_types_number} edges with singleton edge types, which are ",
                            "{singleton_edge_types_list}",
                            "{additional_edgges_with_singleton_edge_types}. "
                        ),
                        singleton_edges_types_number = singleton_edges_types_number,
                        singleton_edge_types_list = get_unchecked_formatted_list(
                            self.iter_singleton_edge_type_names()
                                .unwrap()
                                .take(5)
                                .map(|edge_type_name| {
                                    get_edge_type_source_html_url_from_edge_type_name(
                                        edge_type_name.as_ref(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .as_ref()
                        ),
                        additional_edgges_with_singleton_edge_types =
                            if singleton_edges_types_number > 5 {
                                format!(
                                ", plus other {singleton_edges_types_number} edges with singleton edge types",
                                singleton_edges_types_number = singleton_edges_types_number - 5
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
                "The graph contains {unknown_edge_types_number}, making up {unknown_edge_types_percentage:.2} of the edges.</p>"
            ),
            unknown_edge_types_percentage = self.get_unknown_edge_types_rate().unwrap() * 100.0,
            unknown_edge_types_number = match self.get_unknown_edge_types_number().unwrap() {
                1 => format!(
                    "a edge with unknown edge type, which is {}.",
                    self.get_unchecked_succinct_edge_description(
                        self.iter_edge_ids_with_unknown_edge_types()
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
                            self.iter_edge_ids_with_unknown_edge_types()
                                .unwrap()
                                .take(5)
                                .map(|edge_id| {
                                    self.get_unchecked_succinct_edge_description(edge_id)
                                })
                                .collect::<Vec<_>>()
                                .as_ref()
                        ),
                        additional_unknown_edges = if unknown_types_number > 5 {
                            format!(
                                ", plus other {unknown_types_number} edges with unknown edge types",
                                unknown_types_number = unknown_types_number - 5
                            )
                        } else {
                            "".to_string()
                        }
                    )
                }
            }
        )
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
                "<p>The graph has {edge_types_number}.</p>",
                "<h4>RAM requirements</h4>",
                "<p>The RAM requirements for the edge types data structure is {ram_edge_types}.</p>"
            ),
            edge_types_number = match self.get_edge_types_number().unwrap() {
                1 => format!(
                    concat!(
                        "a single edge type, which is {edge_type_description}. ",
                        "Note that this means that all edges have the same ",
                        "edge type, that is, all edges are homogeneous.",
                    ),
                    edge_type_description = get_edge_type_source_html_url_from_edge_type_name(
                        self.get_edge_type_name_from_edge_type_id(0)
                            .unwrap()
                            .as_ref()
                    )
                ),
                edge_types_number => {
                    let mut edge_type_counts = self
                        .get_edge_type_names_counts_hashmap()
                        .unwrap()
                        .into_iter()
                        .collect::<Vec<_>>();
                    edge_type_counts.sort_by(|(_, a), (_, b)| b.cmp(a));
                    let edge_type_descriptions = get_unchecked_formatted_list(
                        edge_type_counts
                            .into_iter()
                            .take(5)
                            .map(|(edge_type_name, count)| {
                                format!(
                                    "{html_url} ({count} edges, {percentage:.2}%)",
                                    html_url = get_edge_type_source_html_url_from_edge_type_name(
                                        edge_type_name.as_ref()
                                    ),
                                    count = count,
                                    percentage = (count as f64
                                        / self.get_directed_edges_number() as f64)
                                        * 100.0
                                )
                            })
                            .collect::<Vec<_>>()
                            .as_ref(),
                    );
                    format!(
                        "{edge_types_number} edge types, {top_five_caveat} {edge_type_description}",
                        edge_types_number = edge_types_number,
                        top_five_caveat = if edge_types_number > 5 {
                            "of which the 5 most common are"
                        } else {
                            "which are"
                        },
                        edge_type_description = edge_type_descriptions
                    )
                }
            },
            ram_edge_types = self
                .get_edge_types_total_memory_requirements_human_readable()
                .unwrap()
        ));

        // When the graph contains singleton edge types, we build their report.
        if self.has_singleton_edge_types().unwrap() {
            paragraphs.push(self.get_singleton_edges_types_report());
        }

        // When the graph contains unknown edge types, we build their report.
        if self.has_unknown_edge_types().unwrap() {
            paragraphs.push(self.get_unknown_edge_types_report());
        }

        paragraphs.join("")
    }

    /// Return html short textual report of the graph.
    ///
    /// TODO! Add reports on triangles
    /// TODO! Add reports on connected components
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
            // And if the graph is undirected, we have some more high efficiency
            // algorithms that we may want to use.
            if !self.is_directed() {
                // We compute the connected components, if the speed-ups relative to
                // the destinations are enabled (as it may be too slow otherwise on
                // some instances. Similarly, if the graph has less than 1M nodes we
                // also compute the connected components as it should be quite immediate.
                if self.get_nodes_number() < 10e6 as NodeT || self.destinations.is_some() {
                    paragraphs.push(self.get_report_of_connected_components());
                }
                // And the report with oddities, if there are any to report
                if let Some(oddity_report) = self.get_report_of_topological_oddities().unwrap() {
                    paragraphs.push(oddity_report);
                }
            }
        }

        // We add to the report the graph on disconnected nodes if the graph
        // contains any.
        if self.has_disconnected_nodes() {
            paragraphs.push(unsafe { self.get_disconnected_nodes_report() });
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

        let mut report = paragraphs.join("");
        report = report.replace("<p>", "<p align=\"justify\">");
        report
    }
}
