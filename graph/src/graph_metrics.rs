use super::*;
use super::types::*;
use std::collections::HashSet;
use rayon::prelude::*;

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
    pub fn jaccard_index(&self, one:NodeT, two:NodeT)->f64{
        if self.is_node_trap(one) || self.is_node_trap(two){
            return 0.0f64;
        }
        
        let one_neighbors: HashSet<NodeT> = self.get_node_neighbours(one).iter().cloned().collect();
        let two_neighbors: HashSet<NodeT> = self.get_node_neighbours(two).iter().cloned().collect();
        let intersections: HashSet<NodeT> = one_neighbors.intersection(
            &two_neighbors
        ).cloned().collect();

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
    pub fn adamic_adar_index(&self, one: NodeT, two: NodeT)->f64{
        if self.is_node_trap(one) || self.is_node_trap(two){
            return 0.0f64;
        }

        let one_neighbors: HashSet<NodeT> = self.get_node_neighbours(one).iter().cloned().collect();
        let two_neighbors: HashSet<NodeT> = self.get_node_neighbours(two).iter().cloned().collect();
        let intersections: HashSet<NodeT> = one_neighbors.intersection(
            &two_neighbors
        ).cloned().collect();

        intersections.par_iter()
            .filter(
                |node|
                ! self.is_node_trap(**node)
            )
            .map(|node| 
                1.0 / (self.degree(*node) as f64).ln()
            )
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
    pub fn resource_allocation_index(&self, one: NodeT, two: NodeT)->f64{
        if self.is_node_trap(one) || self.is_node_trap(two){
            return 0.0f64;
        }

        let one_neighbors: HashSet<NodeT> = self.get_node_neighbours(one).iter().cloned().collect();
        let two_neighbors: HashSet<NodeT> = self.get_node_neighbours(two).iter().cloned().collect();
        let intersections: HashSet<NodeT> = one_neighbors.intersection(
            &two_neighbors
        ).cloned().collect();

        intersections.par_iter()
            .filter(
                |node|
                ! self.is_node_trap(**node)
            )
            .map(|node| 
                1.0 / self.degree(*node) as f64
            )
            .sum()
    }
}