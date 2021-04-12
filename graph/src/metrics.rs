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
