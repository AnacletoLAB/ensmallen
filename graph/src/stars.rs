use super::*;
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Hash, Clone, Debug, PartialEq)]
pub struct Star {
    graph: Graph,
    root_node_id: NodeT,
    len: NodeT,
}

use std::string::ToString;
impl ToString for Star {
    fn to_string(&self) -> String {
        format!(
            concat!(
                "<p>This star of nodes from the graph {} contains {} nodes, and has as central node {}. ",
                "Specifically, the nodes involved in the star are: {}.</p>",
            ),
            self.graph.get_name(),
            to_human_readable_high_integer(self.len() as usize),
            unsafe{self.graph.get_unchecked_succinct_node_description(self.get_root_node_id())},
            unsafe {
                get_unchecked_formatted_list(
                    &self
                        .get_star_node_ids()
                        .into_iter()
                        .skip(1)
                        .map(|node_id| self.graph.get_unchecked_succinct_node_description(node_id))
                        .collect::<Vec<String>>(),
                        Some(5)
                )
            }
        )
    }
}

impl PartialOrd for Star {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.len.cmp(&other.len))
    }
}

impl Star {
    /// Return new Star object created with the provided root and length.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph of reference of the star.
    /// * `root_node_id`: NodeT - Central node ID of the star.
    /// * `len`: NodeT - precomputed length of the star.
    pub(crate) fn new(graph: &Graph, root_node_id: NodeT, len: NodeT) -> Star {
        Star {
            graph: graph.clone(),
            root_node_id,
            len,
        }
    }

    /// Return the central node ID of the Star.
    pub fn get_root_node_id(&self) -> NodeT {
        self.root_node_id
    }

    /// Return the central node name of the star.
    pub fn get_root_node_name(&self) -> String {
        unsafe {
            self.graph
                .get_unchecked_node_name_from_node_id(self.root_node_id)
        }
    }

    /// Return length of the Star.
    pub fn len(&self) -> NodeT {
        self.len
    }

    /// Return the node IDs of the nodes composing the Star.
    pub fn get_star_node_ids(&self) -> Vec<NodeT> {
        unsafe {
            self.graph
                .get_star_node_ids_from_root_node_id(self.root_node_id)
        }
    }

    /// Return the node names of the nodes composing the star.
    pub fn par_iter_star_node_names(&self) -> impl IndexedParallelIterator<Item = String> + '_ {
        self.get_star_node_ids()
            .into_par_iter()
            .map(move |node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Return the first `k` node IDs of the nodes composing the star.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_star_node_ids(&self, k: usize) -> Vec<NodeT> {
        self.get_star_node_ids().into_iter().take(k).collect()
    }

    /// Return the first `k` node names of the nodes composing the star.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_star_node_names(&self, k: usize) -> Vec<String> {
        self.par_iter_star_node_names().take(k).collect()
    }

    /// Return the node names of the nodes composing the star.
    pub fn get_star_node_names(&self) -> Vec<String> {
        self.par_iter_star_node_names().collect()
    }
}

impl Graph {
    /// Return node IDs in the Star starting from the provided node ID.
    ///
    /// # Arguments
    /// `node_id`: NodeT - The root of the provided Star.
    ///
    /// # Safety
    /// The node ID must be among the node IDs present in the graph, or the method will panic.
    /// Additionally, it must be the root node of a Star.
    unsafe fn get_star_node_ids_from_root_node_id(&self, root_node_id: NodeT) -> Vec<NodeT> {
        let mut result = vec![root_node_id];
        let mut previous_node_id = root_node_id;
        for neighbour_node_id in self.iter_unchecked_neighbour_node_ids_from_source_node_id(root_node_id) {
            if neighbour_node_id != root_node_id && neighbour_node_id != previous_node_id {
                result.push(neighbour_node_id);
                previous_node_id = neighbour_node_id;
            } 
        }

        result
    }

    /// Return vector of Stars in the current graph instance.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_star`: Option<NodeT> - Minimum size of the Stars.
    ///
    /// # Definitions
    /// A star center is a node whose neighbours are connected only to the star center (and possibly themself).
    /// A star node has at least two neighbours.
    /// A star is a component composed by the star center and its neighbours.
    /// 
    /// Note that this definition allows for both self-loops and multigraphs.
    pub fn get_stars(&self, minimum_number_of_nodes_per_star: Option<NodeT>) -> Result<Vec<Star>> {
        self.must_be_undirected()?;
        let minimum_number_of_nodes_per_star = minimum_number_of_nodes_per_star.unwrap_or(10);

        // check for unreasonable inputs
        if minimum_number_of_nodes_per_star <= 2 {
            return Err("A star with less than two neighbours is not a valid definition.".into());
        }

        Ok(self
            .par_iter_node_ids()
            .filter_map(|node_id| unsafe {
                let mut node_degree = 0;
                for neighbour_node_id in
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                {
                    // in a star, the neighbours have only links to the center star node
                    if neighbour_node_id == node_id {
                        continue;
                    }
                    // If a neighbour has other neighbours then it's not a pure star
                    if self
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(neighbour_node_id)
                        .any(|inner_node_id| {
                            inner_node_id != neighbour_node_id && inner_node_id != node_id
                        })
                    {
                        return None;
                    }
                    node_degree += 1;
                }
                // filter out small stars
                if node_degree <= minimum_number_of_nodes_per_star {
                    return None;
                }
                Some(Star::new(self, node_id, 1 + node_degree as NodeT))
            })
            .collect::<Vec<Star>>())
    }
}
