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
    pub fn report(&self) -> HashMap<&str, String> {
        let mut report: HashMap<&str, String> = HashMap::new();

        // Adding the default metrics
        report.insert("name", self.name.clone());
        report.insert("directed", self.is_directed().to_string());
        report.insert("nodes_number", self.get_nodes_number().to_string());
        report.insert(
            "singleton_nodes_number",
            self.get_singleton_nodes_number().to_string(),
        );
        if self.has_nodes() {
            report.insert("density", self.get_density().unwrap().to_string());
            report.insert(
                "minimum_unweighted_node_degree",
                self.get_unweighted_min_node_degree().unwrap().to_string(),
            );
            report.insert(
                "maximum_unweighted_node_degree",
                self.get_unweighted_max_node_degree().unwrap().to_string(),
            );
            report.insert(
                "unweighted_node_degrees_mean",
                self.get_unweighted_node_degrees_mean().unwrap().to_string(),
            );
        }
        report.insert(
            "directed_edges_number",
            self.get_directed_edges_number().to_string(),
        );
        report.insert(
            "selfloops_number",
            self.get_selfloop_nodes_number().to_string(),
        );
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
                self.get_weighted_mininum_node_degree().unwrap().to_string(),
            );
            report.insert(
                "maximum_weighted_node_degree",
                self.get_weighted_maximum_node_degree().unwrap().to_string(),
            );
            report.insert(
                "unweighted_node_degrees_mean",
                self.get_weighted_node_degrees_mean().unwrap().to_string(),
            );
            report.insert(
                "total_edge_weights",
                self.get_total_edge_weights().unwrap().to_string(),
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
    pub fn overlap_textual_report(
        &self,
        other: &Graph,
        verbose: Option<bool>,
    ) -> Result<String, String> {
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

    /// Returns given list in a uman readable format.
    ///
    /// # Safety
    /// If the list is empty the method will raise a panic.
    unsafe fn get_unchecked_formatted_list(&self, list: &[String]) -> String {
        if list.is_empty() {
            panic!("Cannot format a list with no elements.");
        }
        if list.len() == 1 {
            return list.first().unwrap().clone();
        }
        let all_minus_last: String = list[0..list.len() - 1].join(", ");
        format!(
            "{all_minus_last} and {last}",
            all_minus_last = all_minus_last,
            last = list.last().unwrap()
        )
    }

    /// Return human-readable markdown report of the given node.
    ///
    /// The report, by default, is rendered using Markdown.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_id(&self, node_id: NodeT) -> Result<String, String> {
        self.validate_node_id(node_id)?;
        let mut partial_reports: Vec<String> = Vec::new();
        let node_name = unsafe { self.get_unchecked_node_name_from_node_id(node_id) };
        //partial_reports.push(format!("## Report for node {}\n", node_name));

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
            } else if self.is_singleton_with_selfloops_from_node_id(node_id) {
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
                    unsafe { self.get_unchecked_unweighted_node_degree_from_node_id(node_id) }
                )
            },
        );

        Ok(partial_reports.join(""))
    }

    /// Return human-readable markdown report of the given node.
    ///
    /// The report, by default, is rendered using Markdown.
    ///
    /// # Arguments
    /// * `node_name`: &str - Whether to show a loading bar in graph operations.
    ///
    pub fn get_node_report_from_node_name(&self, node_name: &str) -> Result<String, String> {
        self.get_node_id_from_node_name(node_name)
            .and_then(|node_id| self.get_node_report_from_node_id(node_id))
    }

    /// Return human-readable markdown report of the graph peculiarities.
    ///
    /// The report, by default, is rendered using Markdown.
    ///
    pub fn get_peculiarities_report_markdown(&self) -> String {
        let mut partial_reports: Vec<String> = Vec::new();

        partial_reports.push(format!(
            "## Peculiarities report for graph {}\n",
            self.get_name()
        ));

        if !self.has_nodes() {
            partial_reports.push("### Absence of nodes\n".to_string());
            partial_reports.push(
                concat!(
                    "The graph does not have any node. This may be caused by ",
                    "an improper use of one of the filter methods.\n\n"
                )
                .to_string(),
            );
        }

        if !self.has_edges() {
            partial_reports.push("### Absence of edges\n".to_string());
            partial_reports.push(
                concat!(
                    "The graph does not have any edge. This may be caused by ",
                    "an improper use of one of the filter methods.\n\n"
                )
                .to_string(),
            );
        }

        // Detect weirdness relative to nodes
        if self.has_node_oddities() {
            partial_reports.push("### Oddities relative to nodes\n".to_string());
            if self.has_singleton_nodes() {
                partial_reports.push("#### Singleton nodes\n".to_string());
                partial_reports.push(format!(
                    concat!(
                        "{}: nodes that do not have any inbound or outbound edge. ",
                        "We consider singleton nodes an oddity because they represent ",
                        "a concept that is not connected to anything else ",
                        "and is hardly ever useful when actually using the graph.\n",
                        "For instance, in most node embedding methods, the ",
                        "singleton nodes will often maintain a gaussian node ",
                        "embedding, that is often visualized as a gaussian hyper-sphere.\n",
                        "Such embeddings do not encode any information if not the fact ",
                        "that the node has extremely low degree.\n",
                        "\n",
                        "Often these cases are caused by some error in the ",
                        "data wrangling phase. The solutions include, if no bug ",
                        "is identified in the data wrangling phase, to drop",
                        "the singleton nodes or to attach the singletons to ",
                        "other nodes when additional features are available.",
                        "\n",
                        "##### Solution dropping singleton nodes\n",
                        "It is possible to drop **all** of the singleton nodes ",
                        "by using the method `graph.drop_singleton_nodes()`, ",
                        "which will create a new graph instance before removing ",
                        "the singleton nodes.\n",
                        "If you need a more fine-grained control on what is ",
                        "removed, you can use the `filter` method.\n",
                        "##### Solution chaining nodes using k-meas\n",
                        "Another possible solution, when extra node features ",
                        "are available (i.e. when there are word embedding ",
                        "of the nodes description), additional edges may be ",
                        "added to the graph by computing the nodes that are ",
                        "close according to some metric and add edges for the ",
                        "nodes that result to be closer than a given amount ",
                        "in the computed distance.\n",
                        "Add the time of writing this is not supported in ",
                        "Ensmallen, but is work in progress. Currently ",
                        "you will need to handle this in your preprocessing ",
                        "pipeline before providing the edge list."
                    ),
                    match self.get_singleton_nodes_number() {
                        0 => unreachable!(
                            "There must be at least a singleton node if we got here.",
                        ),
                        1 => "There is a singleton node in the graph".to_string(),
                        singleton_node_types_number => format!(
                            "There are {} singleton nodes in the graph",
                            singleton_node_types_number
                        ),
                    }
                ));
                partial_reports.push("##### List of the singleton nodes\n".to_string());
                partial_reports.extend(self.iter_singleton_node_ids().take(10).map(
                    |node_id| unsafe {
                        format!(
                            "* {}\n",
                            self.get_unchecked_succinct_node_description(node_id)
                        )
                    },
                ));
                if self.get_singleton_nodes_number() > 10 {
                    partial_reports.push(format!(
                        "* Plus other {} singleton nodes\n",
                        self.get_singleton_nodes_number() - 10
                    ))
                }
                partial_reports.push("\n".to_string());
            }

            if self.has_singleton_nodes_with_selfloops() {
                partial_reports.push("#### Singleton nodes with self loops\n".to_string());
                partial_reports.push(format!(
                    concat!(
                        "{}: nodes that do not have any inbound or outbound edge, ",
                        "with the exception of one or more selfloops.\n",
                        "We consider singleton nodes with selfloops an oddity because they represent ",
                        "a concept that is not connected to anything else ",
                        "but themselves ",
                        "and is hardly ever useful when actually using the graph.\n",
                        "For instance, in most node embedding methods, the ",
                        "singleton nodes with selfloops will often maintain a gaussian node ",
                        "embedding, that is often visualized as a gaussian hyper-sphere.\n",
                        "Such embeddings do not encode any information if not the fact ",
                        "that the node has extremely low degree, similarly to what ",
                        "happens with a *normal* singleton node.\n",
                        "\n",
                        "Often these cases are caused by some error in the ",
                        "data wrangling phase. The solutions include, if no bug ",
                        "is identified in the data wrangling phase, to drop ",
                        "the singleton nodes with selfloops or to attach these ",
                        "nodes to other nodes when additional features are available.\n",
                        "\n",
                        "##### Solution dropping singleton nodes\n",
                        "It is possible to drop **all** of the singleton nodes with selfloops ",
                        "by using the method `graph.drop_singleton_nodes_with_selfloops()`, ",
                        "which will create a new graph instance before removing ",
                        "the singleton nodes with selfloops.\n",
                        "If you need a more fine-grained control on what is ",
                        "removed, you can use the `filter` method.\n",
                        "##### Solution chaining nodes using k-meas\n",
                        "Another possible solution, when extra node features ",
                        "are available (i.e. when there are word embedding ",
                        "of the nodes description), additional edges may be ",
                        "added to the graph by computing the nodes that are ",
                        "close according to some metric and add edges for the ",
                        "nodes that result to be closer than a given amount ",
                        "in the computed distance.\n",
                        "Add the time of writing this is not supported in ",
                        "Ensmallen, but is work in progress. Currently ",
                        "you will need to handle this in your preprocessing ",
                        "pipeline before providing the edge list."
                    ),
                    match self.get_singleton_nodes_with_selfloops_number() {
                        0 => unreachable!(
                            "There must be at least a singleton node with selfloops if we got here.",
                        ),
                        1 => "There is a singleton node with selfloops in the graph".to_string(),
                        singleton_node_types_number => format!(
                            "There are {} singleton nodes with selfloops in the graph",
                            singleton_node_types_number
                        ),
                    }
                ));
                partial_reports
                    .push("##### List of the singleton nodes with selfloops\n".to_string());
                partial_reports.extend(self.iter_singleton_nodes_with_selfloops_node_ids().take(10).map(
                    |node_id| unsafe {
                        format!(
                            "* {}\n",
                            self.get_unchecked_succinct_node_description(node_id)
                        )
                    },
                ));
                if self.get_singleton_nodes_with_selfloops_number() > 10 {
                    partial_reports.push(format!(
                        "* Plus other {} singleton nodes with selfloops\n",
                        self.get_singleton_nodes_with_selfloops_number() - 10
                    ))
                }
                partial_reports.push("\n".to_string());
            }
        }

        // Detect weirdness relative to node types.
        if self.has_node_types_oddities().map_or(false, |value| value) {
            partial_reports.push("### Oddities relative to node types\n".to_string());
            if self.has_singleton_node_types().unwrap() {
                partial_reports.push("#### Singleton node types\n".to_string());
                partial_reports.push(format!(
                    concat!(
                        "{}: node types that only appear in one graph node. ",
                        "We consider singleton node types an oddity because it ",
                        "identifies a single node uniquely, and the node name ",
                        "already covers that function.\n",
                        "Often these cases are caused by some error in the ",
                        "data wrangling phase when attempting to normalize ",
                        "the node types: consider checking the normalization ",
                        "step and see if these node types fall in one of the other node types.\n",
                        "There are two possible solutions to the peculiarity ",
                        "mentioned above: either drop the singleton node types ",
                        "or replace them with one of the other node types. ",
                        "The first solution may lead to nodes with unknown ",
                        "node types that can be either dropped or imputed.\n",
                        "\n",
                        "##### Solution dropping singleton node types\n",
                        "It is possible to drop **all** of the singleton node ",
                        "types by using the method `graph.remove_inplace_singleton_node_types()`, ",
                        "which will remove *inplace* (from the current instance) ",
                        "all of the singleton node types or, similarly, ",
                        "the method `graph.remove_singleton_node_types()` ",
                        "which will create a new graph instance before removing ",
                        "the singleton node types.\n",
                        "To drop only selected singleton node types you can ",
                        "use one of the following methods, according if you ",
                        "intend to create a new graph instance or not and if ",
                        "you want to execute the operation starting from ",
                        "either the node type ID or the node type name:\n",
                        "* `graph.remove_inplace_node_type_id(node_type_id)`\n",
                        "* `graph.remove_node_type_id(node_type_id)`\n",
                        "* `graph.remove_inplace_node_type_name(node_type_name)`\n",
                        "* `graph.remove_node_type_name(node_type_name)`\n",
                        "\n",
                        "##### Solution replacing singleton node types\n",
                        "An alternative solution is provided by the `replace` ",
                        "method: by providing the desired `node_type_names` ",
                        "parameter you can remap the singleton node types ",
                        "to other node types.\n"
                    ),
                    match self.get_singleton_node_types_number().unwrap() {
                        0 => unreachable!(
                            "There must be at least a singleton node type if we got here.",
                        ),
                        1 => "There is a singleton node type in the graph".to_string(),
                        singleton_node_types_number => format!(
                            "There are {} singleton node types in the graph",
                            singleton_node_types_number
                        ),
                    }
                ));
                partial_reports.push("##### List of the singleton node types\n".to_string());
                partial_reports.extend(
                    self.iter_singleton_node_type_names()
                        .unwrap()
                        .take(10)
                        .map(|node_type_name| {
                            format!(
                                "* {}\n",
                                get_node_type_source_markdown_url_from_node_type_name(
                                    node_type_name.as_ref()
                                )
                            )
                        }),
                );
                if self.get_singleton_node_types_number().unwrap() > 10 {
                    partial_reports.push(format!(
                        "* Plus other {} singleton node types\n",
                        self.get_singleton_node_types_number().unwrap() - 10
                    ))
                }
                partial_reports.push("\n".to_string());
            }
            if self.has_homogeneous_node_types().unwrap() {
                partial_reports.push("#### Homogeneous node types\n".to_string());
                partial_reports.push(
                    concat!(
                        "The current graph instance has homogenous node types. ",
                        "That is, all nodes share the same node type. ",
                        "Graphs with a single node type are odd because if all ",
                        "nodes have the same node type, they might as well have none. ",
                        "A modelling issue often causes this: for instance, ",
                        "when working on a graph such as STRING PPI, a ",
                        "protein-protein interactions graph, it is well known ",
                        "that all nodes represent a protein and hence it would ",
                        "not make sense to add such a node type. Using homogeneous ",
                        "node types only leads to a (slightly) higher memory ",
                        "footprint and slower embedding if your embedding ",
                        "algorithms also involves the node type.\n\n",
                        "Consider avoiding loading homogenous node types ",
                        "altogether or dropping the node types by using either ",
                        "the method `remove_inplace_node_types` or `remove_node_types` ",
                        "to remove the node types in place or creating a ",
                        "new graph instance without the node types.\n"
                    )
                    .to_string(),
                );
            }
            if self.has_unknown_node_types().unwrap() {
                partial_reports.push("#### Unknown node types\n".to_string());
                partial_reports.push(format!(
                    concat!(
                        "The following is less than an oddity and more ",
                        "of a statement: the graph contains {} nodes with ",
                        "unknown node types, composing {:.4} of the nodes.\n",
                        "The presence of unknown node types should be a ",
                        "conscious modelling choice for either actual ",
                        "unknown node types or node types reserved for a ",
                        "validation set of some kind and not related to a ",
                        "data bug created while ingested malformed data sources.\n",
                        "\n",
                        "If you have a sound reason to have unknown node types ",
                        "in your graph then you can absolutely ignore this warning.\n",
                        "Conversely, if you want to remove the unknown node types ",
                        "you can either use the `drop_unknown_node_types` method ",
                        "to drop them and the related nodes, otherwise you can ",
                        "remap the unknown node types to some other node type ",
                        "if you have a generic node type, as is common in most ",
                        "knowledge graphs: you can use the method ",
                        "`replace_unknown_node_types_with_node_type_name` for",
                        "this second solution.\n"
                    ),
                    self.get_unknown_node_types_number().unwrap(),
                    self.get_unknown_node_types_rate().unwrap() * 100.0,
                ));
            }
        }

        // Detect weirdness relative to edge types.
        if self.has_edge_types_oddities().map_or(false, |value| value) {
            partial_reports.push("### Oddities relative to edge types\n".to_string());
            if self.has_singleton_edge_types().unwrap() {
                partial_reports.push("#### Singleton edge types\n".to_string());
                partial_reports.push(format!(
                    concat!(
                        "{}: edge types that only appear in one graph edge. ",
                        "We consider singleton edge types an oddity because it ",
                        "identifies a single edge uniquely, and the edge name ",
                        "already covers that function.\n",
                        "Often these cases are caused by some error in the ",
                        "data wrangling phase when attempting to normalize ",
                        "the edge types: consider checking the normalization ",
                        "step and see if these edge types fall in one of the other edge types.\n",
                        "There are two possible solutions to the peculiarity ",
                        "mentioned above: either drop the singleton edge types ",
                        "or replace them with one of the other edge types. ",
                        "The first solution may lead to edges with unknown ",
                        "edge types that can be either dropped or imputed.\n",
                        "\n",
                        "##### Solution dropping singleton edge types\n",
                        "It is possible to drop **all** of the singleton edge ",
                        "types by using the method `graph.remove_inplace_singleton_edge_types()`, ",
                        "which will remove *inplace* (from the current instance) ",
                        "all of the singleton edge types or, similarly, ",
                        "the method `graph.remove_singleton_edge_types()` ",
                        "which will create a new graph instance before removing ",
                        "the singleton edge types.\n",
                        "To drop only selected singleton edge types you can ",
                        "use one of the following methods, according if you ",
                        "intend to create a new graph instance or not and if ",
                        "you want to execute the operation starting from ",
                        "either the edge type ID or the edge type name:\n",
                        "* `graph.remove_inplace_edge_type_id(edge_type_id)`\n",
                        "* `graph.remove_edge_type_id(edge_type_id)`\n",
                        "* `graph.remove_inplace_edge_type_name(edge_type_name)`\n",
                        "* `graph.remove_edge_type_name(edge_type_name)`\n",
                        "\n",
                        "##### Solution replacing singleton edge types\n",
                        "An alternative solution is provided by the `replace` ",
                        "method: by providing the desired `edge_type_names` ",
                        "parameter you can remap the singleton edge types ",
                        "to other edge types.\n"
                    ),
                    match self.get_singleton_edge_types_number().unwrap() {
                        0 => unreachable!(
                            "There must be at least a singleton edge type if we got here.",
                        ),
                        1 => "There is a singleton edge type in the graph".to_string(),
                        singleton_edge_types_number => format!(
                            "There are {} singleton edge types in the graph",
                            singleton_edge_types_number
                        ),
                    }
                ));
                partial_reports.push("##### List of the singleton edge types\n".to_string());
                partial_reports.extend(
                    self.iter_singleton_edge_type_names()
                        .unwrap()
                        .take(10)
                        .map(|edge_type_name| {
                            format!(
                                "* {}\n",
                                get_node_type_source_markdown_url_from_node_type_name(
                                    edge_type_name.as_ref()
                                )
                            )
                        }),
                );
                if self.get_singleton_edge_types_number().unwrap() > 10 {
                    partial_reports.push(format!(
                        "* Plus other {} singleton edge types\n",
                        self.get_singleton_edge_types_number().unwrap() - 10
                    ))
                }
                partial_reports.push("\n".to_string());
            }
            if self.has_homogeneous_edge_types().unwrap() {
                partial_reports.push("#### Homogeneous edge types\n".to_string());
                partial_reports.push(
                    concat!(
                        "The current graph instance has homogenous edge types. ",
                        "That is, all edges share the same edge type. ",
                        "Graphs with a single edge type are odd because if all ",
                        "edges have the same edge type, they might as well have none. ",
                        "A modelling issue often causes this: for instance, ",
                        "when working on a graph such as STRING PPI, a ",
                        "protein-protein interactions graph, it is well known ",
                        "that all edges represent a protein and hence it would ",
                        "not make sense to add such a edge type. Using homogeneous ",
                        "edge types only leads to a (slightly) higher memory ",
                        "footprint and slower embedding if your embedding ",
                        "algorithms also involves the edge type.\n\n",
                        "Consider avoiding loading homogenous edge types ",
                        "altogether or dropping the edge types by using either ",
                        "the method `remove_inplace_edge_types` or `remove_edge_types` ",
                        "to remove the edge types in place or creating a ",
                        "new graph instance without the edge types.\n"
                    )
                    .to_string(),
                );
            }
            if self.has_unknown_edge_types().unwrap() {
                partial_reports.push("#### Unknown edge types\n".to_string());
                partial_reports.push(format!(
                    concat!(
                        "The following is less than an oddity and more ",
                        "of a statement: the graph contains {} edges with ",
                        "unknown edge types, composing {:.4} of the edges.\n",
                        "The presence of unknown edge types should be a ",
                        "conscious modelling choice for either actual ",
                        "unknown edge types or edge types reserved for a ",
                        "validation set of some kind and not related to a ",
                        "data bug created while ingested malformed data sources.\n",
                        "\n",
                        "If you have a sound reason to have unknown edge types ",
                        "in your graph then you can absolutely ignore this warning.\n",
                        "Conversely, if you want to remove the unknown edge types ",
                        "you can either use the `drop_unknown_edge_types` method ",
                        "to drop them and the related edges, otherwise you can ",
                        "remap the unknown edge types to some other edge type ",
                        "if you have a generic edge type, as is common in most ",
                        "knowledge graphs: you can use the method ",
                        "`replace_unknown_edge_types_with_edge_type_name` for",
                        "this second solution.\n"
                    ),
                    self.get_unknown_edge_types_number().unwrap(),
                    self.get_unknown_edge_types_rate().unwrap() * 100.0,
                ));
            }
        }

        // If there is only the title, then we have not detected any weirdness.
        if partial_reports.len() == 1 {
            partial_reports.push(format!(
                "Congratulations, the graph {} does not seem to have any weirdness!\n",
                self.get_name()
            ));
        }

        partial_reports.join("")
    }

    /// Returns markdown formatting for the given node name URLs.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - Node ID to query for.
    ///
    /// # Safety
    /// This method will cause an out of bound if the given node ID does not exist.
    unsafe fn get_unchecked_succinct_node_description(&self, node_id: NodeT) -> String {
        let node_name = self.get_unchecked_node_name_from_node_id(node_id);
        let node_name = get_node_source_markdown_url_from_node_name(node_name.as_ref());
        let node_degree = self.get_unchecked_unweighted_node_degree_from_node_id(node_id);
        let node_type = if self.has_node_types() {
            match self.get_unchecked_node_type_names_from_node_id(0) {
                Some(node_type_names) => match node_type_names.len() {
                    1 => Some(format!(
                        "node type {}",
                        get_node_type_source_markdown_url_from_node_type_name(
                            node_type_names.first().unwrap().as_ref()
                        )
                    )),
                    _ => Some(format!(
                        "node types {}",
                        self.get_unchecked_formatted_list(
                            node_type_names
                                .iter()
                                .map(|node_type_name| {
                                    get_node_type_source_markdown_url_from_node_type_name(
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
        let mut node_degree = match self.get_unweighted_node_degree_from_node_id(node_id) {
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
                    weighted_degree =
                        self.get_unchecked_unweighted_node_degree_from_node_id(node_id)
                )
            });
        }

        // If any of the terms was given we build the output description
        let description = if node_degree.is_some() || node_type.is_some() {
            format!(
                "({node_degree}{join_term}{node_type})",
                node_degree = node_degree.unwrap_or("".to_string()),
                join_term = if node_degree.is_some() && node_type.is_some() {
                    " and "
                } else {
                    ""
                },
                node_type = node_type.unwrap_or("".to_string())
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

    /// Return markdown short textual report of the graph.
    ///
    /// # Implementative details
    /// This textual report is meant to give a brief sneak peak of the graph
    /// structure, with the very minimal informations that can be computed
    /// in a blink of an eye. In order to get a rich graph report, use the
    /// `complete_textual_report` method.
    pub fn short_textual_report(&self) -> String {
        let name = if self.has_default_graph_name() {
            "".to_string()
        } else {
            format!(" {}", self.get_name())
        };
        if !self.has_nodes() {
            return format!(
                concat!(
                    "The graph{name} is *empty*, that is has neither nodes nor edges.\n",
                    "If this is unexpected, it may have happened because of a ",
                    "mis-parametrization of a filter method uphill."
                ),
                name = name
            );
        }
        let nodes_number = unsafe {
            match self.get_nodes_number() {
                1 => format!(
                    "a single node called {node_name_description}",
                    node_name_description = self.get_unchecked_succinct_node_description(0),
                ),
                nodes_number => format!("{nodes_number} nodes", nodes_number = nodes_number),
            }
        };

        if !self.has_edges() {
            return format!(
                concat!(
                    "The graph{name} contains {nodes_number} and no edges.\n",
                    "If this is unexpected, it may have happened because of a ",
                    "mis-parametrization of a filter method uphill."
                ),
                name = name,
                nodes_number = nodes_number
            );
        }

        let edges_number = unsafe {
            match self.get_edges_number() {
                1 => format!(
                    "a single edge between the source node {source_node_description:?} and the destination node {destination_node_description:?}{edge_type}",
                    source_node_description = self.get_unchecked_succinct_node_description(self.get_unchecked_source_node_id_from_edge_id(0)),
                    destination_node_description = self.get_unchecked_succinct_node_description(self.get_unchecked_destination_node_id_from_edge_id(0)),
                    edge_type = if self.has_edge_types() {
                        match self.get_edge_type_name_from_edge_id(0).unwrap() {
                            Some(edge_type_name) => {
                                format!(
                                    " with edge type {}",
                                    get_edge_type_source_markdown_url_from_edge_type_name(edge_type_name.as_ref())
                                )
                            },
                            None => " with unknown edge type".to_string(),
                        }
                    } else {
                        "".to_string()
                    }
                ),
                edges_number => format!("{edges_number} edges", edges_number = edges_number),
            }
        };

        let most_central_nodes = unsafe {
            format!(
                concat!(
                    "### Degree centrality\n",
                    "The minimum node degree is {minimum_node_degree}, the maximum node degree is {maximum_node_degree}, ",
                    "the mode degree is {mode_node_degree}, the mean degree is {mean_node_degree} and the node degree median is {node_degree_median}.\n",
                    "The nodes with highest degree centrality are: {list_of_most_central_nodes}.\n"
                ),
                minimum_node_degree = self.get_unweighted_min_node_degree().unwrap(),
                maximum_node_degree = self.get_unweighted_max_node_degree().unwrap(),
                mode_node_degree = self.get_unweighted_node_degrees_mode().unwrap(),
                mean_node_degree = self.get_unweighted_node_degrees_mean().unwrap(),
                node_degree_median = self.get_unweighted_node_degrees_median().unwrap(),
                list_of_most_central_nodes = self.get_unchecked_formatted_list(
                    self.get_unweighted_top_k_central_node_ids(5)
                        .into_iter()
                        .filter(|node_id| {
                            self.get_unchecked_unweighted_node_degree_from_node_id(*node_id) > 0
                        })
                        .map(|node_id| {
                            self.get_unchecked_succinct_node_description(node_id)
                        })
                        .collect::<Vec<_>>()
                        .as_ref()
                )
            )
        };

        let disconnected_nodes = unsafe {
            if self.has_disconnected_nodes() {
                let mut disconnected_nodes_report = Vec::new();
                let disconnected_nodes_suggestions = concat!(
                    "These nodes are hard to account ",
                    "for during the computation of graph embedding or ",
                    "when executing predictions using any model ",
                    "based on topological informations.\n",
                    "The possible solutions to handle the singleton nodes ",
                    "include, but are not limited to:\n",
                    "* removing them using `graph.drop_disconnected_nodes()`\n",
                    "* adding selfloops using `graph.add_selfloops()`\n",
                    "* imputing new edges via additional features using `graph.generate_new_edges_from_node_features(features)`\n",
                    "* merge this graph with other related graphs \n"
                );
                disconnected_nodes_report.push(
                    format!(
                        concat!(
                            "### Disconnected nodes\n",
                            "Disconnected nodes are nodes that are not connected ",
                            "to any other node.",
                            "{disconnected_nodes_suggestions}",
                            "The graph contains {disconnected_nodes_number}.\n"
                        ),
                        disconnected_nodes_number = self.get_disconnected_nodes_number(),
                        disconnected_nodes_suggestions = disconnected_nodes_suggestions
                    )
                );
                if self.has_singleton_nodes() {
                    disconnected_nodes_report.push(format!(
                        concat!(
                            "#### Singleton nodes\n",
                            "Singleton nodes are nodes with no edge to other nodes ",
                            "nor selfloops.\n",
                            "The graph contains {singleton_nodes_number}\n."
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
                                        "{additional_singleton_nodes}\n"
                                    ),
                                    singleton_nodes_number = singleton_nodes_number,
                                    singleton_nodes_list = self.get_unchecked_formatted_list(
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
                                            ", plus other {singleton_nodes_number} singleton nodes.",
                                            singleton_nodes_number = singleton_nodes_number - 5
                                        )
                                    } else {
                                        ".".to_string()
                                    }
                                )
                            }
                        }
                    ));
                }
                if self.has_singleton_nodes_with_selfloops() {
                    disconnected_nodes_report.push(format!(
                        concat!(
                            "#### Singleton nodes with selfloops\n",
                            "Singleton nodes with selfloops are nodes with no edge to other nodes ",
                            "and have exclusively selfloops.\n",
                            "The graph contains {singleton_nodes_with_selfloops_number}"
                        ),
                        singleton_nodes_with_selfloops_number = match self.get_singleton_nodes_with_selfloops_number() {
                            1 => format!(
                                "a singleton node with selfloop, which is {}.",
                                self.get_unchecked_succinct_node_description(
                                    self.iter_singleton_nodes_with_selfloops_node_ids().next().unwrap()
                                )
                            ),
                            singleton_nodes_with_selfloops_number => {
                                format!(
                                    concat!(
                                        "{singleton_nodes_with_selfloops_number} singleton nodes with selfloops, which are ",
                                        "{singleton_nodes_list}",
                                        "{additional_singleton_nodes_with_selfloop}.\n"
                                    ),
                                    singleton_nodes_with_selfloops_number = singleton_nodes_with_selfloops_number,
                                    singleton_nodes_list = self.get_unchecked_formatted_list(
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
                                            ", plus other {singleton_nodes_with_selfloops_number} singleton nodes",
                                            singleton_nodes_with_selfloops_number = singleton_nodes_with_selfloops_number - 5
                                        )
                                    } else {
                                        "".to_string()
                                    }
                                )
                            }
                        }
                    ));
                }
                disconnected_nodes_report.join("")
            } else {
                "".to_string()
            }
        };

        let weights = unsafe {
            if self.has_edge_weights() {
                format!(
                    concat!(
                        "### Weights\n",
                        "The minimum edge weight is {minimum_edge_weight}, the maximum edge weight is {maximum_edge_weight} and the total edge weight is {total_edge_weight}.\n",
                        "### Weighted degree centrality\n",
                        "The minimum node degree is {weighted_minimum_node_degree}, the maximum node degree is {weighted_maximum_node_degree}, ",
                        "the mean degree is {weighted_mean_node_degree} and the node degree median is {weighted_node_degree_median}.\n",
                        "The nodes with highest degree centrality are: {weighted_list_of_most_central_nodes}.\n"
                    ),
                    minimum_edge_weight= self.get_mininum_edge_weight().unwrap(),
                    maximum_edge_weight= self.get_mininum_edge_weight().unwrap(),
                    total_edge_weight=self.get_total_edge_weights().unwrap(),
                    weighted_minimum_node_degree = self.get_weighted_mininum_node_degree().unwrap(),
                    weighted_maximum_node_degree = self.get_weighted_maximum_node_degree().unwrap(),
                    weighted_mean_node_degree = self.get_weighted_node_degrees_mean().unwrap(),
                    weighted_node_degree_median = self.get_weighted_node_degrees_median().unwrap(),
                    weighted_list_of_most_central_nodes = self.get_unchecked_formatted_list(
                        self.get_weighted_top_k_central_node_ids(5).unwrap()
                            .into_iter()
                            .filter(|node_id| {
                                self.get_unchecked_unweighted_node_degree_from_node_id(*node_id) > 0
                            })
                            .map(|node_id| {
                                self.get_unchecked_succinct_node_description(node_id)
                            })
                            .collect::<Vec<_>>()
                            .as_ref()
                    )
                )
            } else {
                "".to_string()
            }
        };

        let node_types = if self.has_node_types() {
            unsafe {
                format!(
                    concat!("### Node types\n", "The graph has {node_types_number}.\n"),
                    node_types_number = match self.get_node_types_number().unwrap() {
                        1 => format!(
                            "a single node type, which is {node_type_description}",
                            node_type_description =
                                get_node_type_source_markdown_url_from_node_type_name(
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
                            node_type_counts.par_sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
                            let node_type_descriptions = self.get_unchecked_formatted_list(
                                    node_type_counts.into_iter().take(5)
                                    .map(|(node_type_name, count)| {
                                        format!(
                                            "{markdown_url} ({count} nodes)",
                                            markdown_url=get_node_type_source_markdown_url_from_node_type_name(node_type_name.as_ref()),
                                            count=count
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .as_ref()
                            );
                            format!(
                                "{node_types_number} node types, {top_five_caveat} {node_type_description}",
                                node_types_number = node_types_number,
                                top_five_caveat= if node_types_number > 5 {
                                    "of which the 5 most common are"
                                } else {
                                    "which are"
                                },
                                node_type_description = node_type_descriptions
                            )
                        }
                    }
                )
            }
        } else {
            "".to_string()
        };

        let unknown_node_types = if self.has_node_types() && self.has_unknown_node_types().unwrap()
        {
            format!(
                concat!(
                    "#### Unknown node types\n",
                    "The graph contains {unknown_node_types_number} unknown node types, making up the {unknown_node_types_rate:.2}% of the nodes.\n",
                ),
                unknown_node_types_number = self.get_unknown_node_types_number().unwrap(),
                unknown_node_types_rate = self.get_unknown_node_types_rate().unwrap()*100.0,
            )
        } else {
            "".to_string()
        };

        let edge_types = if self.has_edge_types() {
            unsafe {
                format!(
                    concat!("### edge types\n", "The graph has {edge_types_number}.\n"),
                    edge_types_number = match self.get_edge_types_number().unwrap() {
                        1 => format!(
                            "a single edge type, which is {edge_type_description}",
                            edge_type_description =
                                get_edge_type_source_markdown_url_from_edge_type_name(
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
                            edge_type_counts.par_sort_unstable_by(|(_, a), (_, b)| b.cmp(a));
                            let edge_type_descriptions = self.get_unchecked_formatted_list(
                                    edge_type_counts.into_iter().take(5)
                                    .map(|(edge_type_name, count)| {
                                        format!(
                                            "{markdown_url} ({count} edges)",
                                            markdown_url=get_edge_type_source_markdown_url_from_edge_type_name(edge_type_name.as_ref()),
                                            count=count
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .as_ref()
                            );
                            format!(
                                "{edge_types_number} edge types, {top_five_caveat} {edge_type_description}",
                                edge_types_number = edge_types_number,
                                top_five_caveat= if edge_types_number > 5 {
                                    "of which the 5 most common are" 
                                } else {
                                    "which are"
                                },
                                edge_type_description = edge_type_descriptions
                            )
                        }
                    }
                )
            }
        } else {
            "".to_string()
        };

        let unknown_edge_types = if self.has_edge_types() && self.has_unknown_edge_types().unwrap()
        {
            format!(
                concat!(
                    "#### Unknown edge types\n",
                    "The graph contains {unknown_edge_types_number} unknown edge types, making up the {unknown_edge_types_rate:.2}% of the edges.\n",
                ),
                unknown_edge_types_number = self.get_unknown_edge_types_number().unwrap(),
                unknown_edge_types_rate = self.get_unknown_edge_types_rate().unwrap()*100.0,
            )
        } else {
            "".to_string()
        };

        format!(
            concat!(
                "## Graph{name} report summary\n",
                "The {directionality}{multigraph} graph{name} has {nodes_number} and {edges_number}.\n",
                "{most_central_nodes}",
                "{disconnected_nodes}",
                "{weights}",
                "{node_types}",
                "{unknown_node_types}",
                "{edge_types}",
                "{unknown_edge_types}",
                "\n",
                "*To get a more exaustive report including also more ",
                "computationally expensive graph properties, ",
                "use the `graph.complete_textual_report()` method.*"
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
            name = name,
            nodes_number = nodes_number,
            edges_number = edges_number,
            most_central_nodes = most_central_nodes,
            disconnected_nodes = disconnected_nodes,
            weights = weights,
            node_types = node_types,
            unknown_node_types = unknown_node_types,
            edge_types = edge_types,
            unknown_edge_types = unknown_edge_types
        )
    }

    /// Return markdown complete textual report of the graph.
    ///
    /// # Arguments
    /// *
    /// * `verbose`: Option<bool> - Whether to show loading bar.
    pub fn complete_textual_report(&self, verbose: Option<bool>) -> String {
        let mut partial_reports: Vec<String> = Vec::new();

        partial_reports.push(format!(
            "## Peculiarities report for graph {}\n",
            self.get_name()
        ));

        // Basic report
        if self.has_nodes() {
            partial_reports.push("### Graph nodes\n".to_string());
            partial_reports.push(concat!("The graph contains {} ").to_string());
        }

        // Singleton nodes
        // Singleton nodes with self-loops
        // Weighted singleton nodes
        // Multigraph
        // Node types
        // Edge types

        // Connected components report

        // Triangles, clustering and transitivity

        partial_reports.join("")
    }
}
