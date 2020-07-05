use super::types::*;
use super::*;
use rayon::prelude::*;
use itertools::Itertools;
use hashbrown::{HashMap, HashSet};
use std::collections::{HashMap as DefaultHashMap};

/// Properties and measurements of the graph
impl Graph {
    /// Returns product of degrees of given nodes.
    ///
    /// # Arguments
    ///
    /// * `one` - Integer ID of the first node.
    /// * `two` - Integer ID of the second node.
    ///
    pub fn degrees_product(&self, one: NodeT, two: NodeT) -> usize {
        self.degree(one) as usize * self.degree(two) as usize
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
    pub fn jaccard_index(&self, one: NodeT, two: NodeT) -> f64 {
        if self.is_node_trap(one) || self.is_node_trap(two) {
            return 0.0f64;
        }

        let one_neighbors: HashSet<NodeT> = self.get_node_neighbours(one).iter().cloned().collect();
        let two_neighbors: HashSet<NodeT> = self.get_node_neighbours(two).iter().cloned().collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        intersections.len() as f64 / (one_neighbors.len() + two_neighbors.len()) as f64
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
    pub fn adamic_adar_index(&self, one: NodeT, two: NodeT) -> f64 {
        if self.is_node_trap(one) || self.is_node_trap(two) {
            return 0.0f64;
        }

        let one_neighbors: HashSet<NodeT> = self.get_node_neighbours(one).iter().cloned().collect();
        let two_neighbors: HashSet<NodeT> = self.get_node_neighbours(two).iter().cloned().collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        intersections
            .par_iter()
            .filter(|node| !self.is_node_trap(**node))
            .map(|node| 1.0 / (self.degree(*node) as f64).ln())
            .sum()
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
    pub fn resource_allocation_index(&self, one: NodeT, two: NodeT) -> f64 {
        if self.is_node_trap(one) || self.is_node_trap(two) {
            return 0.0f64;
        }

        let one_neighbors: HashSet<NodeT> = self.get_node_neighbours(one).iter().cloned().collect();
        let two_neighbors: HashSet<NodeT> = self.get_node_neighbours(two).iter().cloned().collect();
        let intersections: HashSet<NodeT> = one_neighbors
            .intersection(&two_neighbors)
            .cloned()
            .collect();

        intersections
            .par_iter()
            .filter(|node| !self.is_node_trap(**node))
            .map(|node| 1.0 / self.degree(*node) as f64)
            .sum()
    }

    /// Returns the traps rate of the graph.
    ///
    /// THIS IS EXPERIMENTAL AND MUST BE PROVEN!
    ///
    pub fn traps_rate(&self) -> f64 {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| {
                if !self.is_node_trap(node) {
                    let neighbours = self.get_node_neighbours(node);
                    neighbours
                        .iter()
                        .map(|n| self.is_node_trap(*n) as usize as f64)
                        .sum::<f64>()
                        / neighbours.len() as f64
                } else {
                    1.0
                }
            })
            .sum::<f64>()
            / self.get_nodes_number() as f64
    }

    /// Returns mean node degree of the graph.
    pub fn degrees_mean(&self) -> f64 {
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.degree(node))
            .sum::<usize>() as f64
            / self.get_nodes_number() as f64
    }

    /// Returns median node degree of the graph
    pub fn degrees_median(&self) -> NodeT {
        let mut degrees = self.degrees();
        degrees.par_sort_unstable();
        degrees[self.get_nodes_number() / 2]
    }

    /// Returns mode node degree of the graph
    pub fn degrees_mode(&self) -> NodeT {
        let mut occurrences: HashMap<NodeT, usize> = HashMap::new();

        for value in self.degrees() {
            *occurrences.entry(value).or_insert(0) += 1;
        }

        occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .unwrap()
    }

    /// Returns percentage of self-loops.
    pub fn selfloops_percentage(&self)->f64{
        (0..self.get_nodes_number())
            .into_par_iter()
            .map(|node| self.has_edge(node, node) as usize)
            .sum::<usize>() as f64 / self.get_edges_number() as f64
    }

    /// Returns percentage of bidirectional edges.
    pub fn bidirectional_percentage(&self)->f64{
        self.unique_edges
            .par_keys()
            .map(|(src, dst)| self.has_edge(*dst, *src) as usize)
            .sum::<usize>() as f64 / self.unique_edges.len() as f64
    }

    /// Returns number of connected components in graph.
    pub fn connected_components_number(&self)->NodeT{
        self.get_nodes_number() - self.spanning_tree(0).len() 
    }

    /// Returns number of singleton nodes within the graph.
    pub fn singleton_nodes_number(&self)->NodeT{
        self.get_nodes_number() - self.destinations
            .iter()
            .chain(
                self.sources.iter()
            )
            .unique()
            .count()
    }

    /// Returns density of the graph.
    pub fn density(&self)->f64{
        self.get_edges_number() as f64 / (self.get_nodes_number().pow(2)) as f64
    }

    /// Returns report relative to the graph metrics
    ///
    /// The report includes a few useful metrics like:
    ///
    /// * degrees_median: the median degree of the nodes.
    /// * degrees_mean: the mean degree of the nodes.
    /// * degrees_mode: the mode degree of the nodes.
    /// * nodes_number: the number of nodes in the graph.
    /// * edges_number: the number of edges in the graph.
    /// * unique_node_types_number: the number of different node types in the graph.
    /// * unique_edge_types_number: the number of different edge types in the graph.
    /// * traps_rate: probability to end up in a trap when starting into any given node.
    /// * selfloops_percentage: pecentage of edges that are selfloops.
    /// * bidirectional_percentage: percentage of edges that are bidirectional.
    ///
    pub fn report(&self) -> DefaultHashMap<&str, String> {
        let mut report: DefaultHashMap<&str, String> = DefaultHashMap::new();
        report.insert("degrees_median", self.degrees_median().to_string());
        report.insert("degrees_mean", self.degrees_mean().to_string());
        report.insert("degrees_mode", self.degrees_mode().to_string());
        report.insert("nodes_number", self.get_nodes_number().to_string());
        report.insert("edges_number", self.get_edges_number().to_string());
        report.insert("density", self.density().to_string());
        report.insert("singleton_nodes", self.singleton_nodes_number().to_string());
        report.insert("is_directed", self.is_directed.to_string());
        report.insert(
            "unique_node_types_number",
            self.get_node_types_number().to_string(),
        );
        report.insert(
            "unique_edge_types_number",
            self.get_edge_types_number().to_string(),
        );
        report.insert("traps_rate", self.traps_rate().to_string());
        report.insert(
            "selfloops_percentage",
            self.selfloops_percentage().to_string()
        );
        report.insert(
            "bidirectional_percentage",
            self.bidirectional_percentage().to_string()
        );
        report.insert(
            "connected_components_number",
            self.connected_components_number().to_string()
        );
        report.insert(
            "strongly_connected_components_number",
            self.strongly_connected_components().len().to_string()
        );
        report
    }
}
