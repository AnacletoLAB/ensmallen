use super::types::*;
use super::*;
use rayon::prelude::*;
use std::collections::HashMap as DefaultHashMap;
use std::collections::{HashMap, HashSet};

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
        Ok(self.get_node_degree(one) as usize * self.get_node_degree(two) as usize)
    }

    pub fn is_source(&self, candidate_source: NodeT) -> bool {
        self.get_node_degree(candidate_source) != 0
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

        if self.is_node_trap(one) || self.is_node_trap(two) {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.get_source_destinations_range(one).collect();
        let two_neighbors: HashSet<NodeT> = self.get_source_destinations_range(two).collect();
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

        if self.is_node_trap(one) || self.is_node_trap(two) {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.get_source_destinations_range(one).collect();
        let two_neighbors: HashSet<NodeT> = self.get_source_destinations_range(two).collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        Ok(intersections
            .par_iter()
            .filter(|node| !self.is_node_trap(**node))
            .map(|node| 1.0 / (self.get_node_degree(*node) as f64).ln())
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

        if self.is_node_trap(one) || self.is_node_trap(two) {
            return Ok(0.0f64);
        }

        let one_neighbors: HashSet<NodeT> = self.get_source_destinations_range(one).collect();
        let two_neighbors: HashSet<NodeT> = self.get_source_destinations_range(two).collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        Ok(intersections
            .par_iter()
            .filter(|node| !self.is_node_trap(**node))
            .map(|node| 1.0 / self.get_node_degree(*node) as f64)
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
                if !self.is_node_trap(node) {
                    self.get_source_destinations_range(node)
                        .map(|dst| self.is_node_trap(dst) as usize as f64)
                        .sum::<f64>()
                        / self.get_node_degree(node) as f64
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
    /// println!("The mean node degree of the graph is  {}", graph.degrees_mean());
    /// ```
    pub fn degrees_mean(&self) -> f64 {
        self.get_edges_number() as f64 / self.get_nodes_number() as f64
    }

    /// Returns median node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The median node degree of the graph is  {}", graph.degrees_median());
    /// ```
    pub fn degrees_median(&self) -> NodeT {
        let mut degrees = self.get_node_degrees();
        degrees.par_sort_unstable();
        degrees[(self.get_nodes_number() / 2) as usize]
    }

    /// Returns maximum node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The maximum node degree of the graph is  {}", graph.degrees_max());
    /// ```
    pub fn degrees_max(&self) -> NodeT {
        *self.get_node_degrees().iter().max().unwrap()
    }

    /// Returns minimum node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The minimum node degree of the graph is  {}", graph.degrees_min());
    /// ```
    pub fn degrees_min(&self) -> NodeT {
        *self.get_node_degrees().iter().min().unwrap()
    }

    /// Returns mode node degree of the graph
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The mode node degree of the graph is  {}", graph.degrees_mode());
    /// ```
    pub fn degrees_mode(&self) -> NodeT {
        let mut occurrences: HashMap<NodeT, usize> = HashMap::new();

        for value in self.get_node_degrees() {
            *occurrences.entry(value).or_insert(0) += 1;
        }

        occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .unwrap()
    }

    /// Returns number of self-loops.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of self-loops in the graph is  {}", graph.get_selfloops_number());
    /// ```
    pub fn get_selfloops_number(&self) -> usize {
        self.get_selfloops_iter().count()
    }

    /// Returns rate of self-loops.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The rate of self-loops in the graph is  {}", graph.get_selfloops_rate());
    /// ```
    pub fn get_selfloops_rate(&self) -> f64 {
        self.get_selfloops_number() as f64 / self.get_edges_number() as f64
    }

    /// Returns rate of bidirectional edges.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The rate of bidirectional edges in the graph is  {}", graph.bidirectional_rate());
    /// ```
    pub fn bidirectional_rate(&self) -> f64 {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.has_edge(node, node) as usize)
            .sum::<usize>() as f64
            / self.edges.len() as f64
    }

    /// Returns number of connected components in graph.
    /// If the graph is or isn't a multigraph the edge types are not considered; if any edge exists, it is considered
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// let n = graph.get_mean_number_of_types_for_edge();
    /// if n > 1.0 {
    ///     println!("The rate of connected components  in the multigraph is  {}", graph.connected_components_number());
    /// }else{
    ///     println!("The rate of connected components in the graph is {} ", graph.connected_components_number());
    /// }
    /// ```
    /// note that, for understanding whether graph is a multigraph, instead of computing the mean number of edge types in the graph (n) and checking that n>1
    /// we could directly use the function is_multigraph(&self):
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// if graph.is_multigraph() {
    ///     println!("The rate of connected components  in the multigraph is  {}", graph.connected_components_number());
    /// }else{
    ///     println!("The rate of connected components in the graph is {} ", graph.connected_components_number());
    /// }
    /// ```
    pub fn connected_components_number(&self) -> NodeT {
        self.get_nodes_number() - self.spanning_tree(0, false, false).len() as NodeT
    }

    /// Returns number of singleton nodes within the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph contains {} singleton nodes", graph.singleton_nodes_number());
    /// ```
    pub fn get_singleton_nodes_number(&self) -> NodeT {
        self.get_nodes_number() - self.get_not_singleton_nodes_number()
    }

    /// Returns number of not singleton nodes within the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph contains {} not singleton nodes", graph.get_not_singleton_nodes_number());
    /// ```
    pub fn get_not_singleton_nodes_number(&self) -> NodeT {
        self.get_not_singletons().len() as NodeT
    }

    /// Returns density of the graph.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The graph density is {}", graph.density());
    /// ```
    pub fn density(&self) -> f64 {
        self.get_edges_number() as f64 / (self.get_nodes_number().pow(2)) as f64
    }

    /// Returns the number of edges that have multiple types.
    ///```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The number of edge with multiple types is {}", graph.get_multigraph_edges_number());
    /// ```
    pub fn get_multigraph_edges_number(&self) -> usize {
        self.get_unique_edges_iter()
            .filter(|(src, dst)| self.get_unchecked_edge_types_number_from_tuple(*src, *dst) > 1)
            .count()
    }

    /// Returns the ratio r_multi of edges that have multiple types; r_multi = (number of edges having multiple types)/(number of edges)
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false).unwrap();
    /// println!("The percentage of edges having multiple types is {}", graph.get_multigraph_edges_ratio());
    /// ```
    pub fn get_multigraph_edges_ratio(&self) -> f64 {
        self.get_multigraph_edges_number() as f64 / self.get_edges_number() as f64
    }

    /// Returns report relative to the graph metrics
    ///
    /// The report includes a few useful metrics like:
    ///
    /// * degrees_median: the median degree of the nodes.
    /// * degrees_mean: the mean degree of the nodes.
    /// * degrees_mode: the mode degree of the nodes.
    /// * degrees_max: the max degree of the nodes.
    /// * degrees_min: the min degree of the nodes.
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
        report.insert("nodes_number", self.get_nodes_number().to_string());
        report.insert("edges_number", self.get_edges_number().to_string());
        report.insert("density", self.density().to_string());
        report.insert("directed", self.directed.to_string());
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
}
