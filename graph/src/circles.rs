use super::*;
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Hash, Clone, Debug, PartialEq)]
pub struct Circle {
    graph: Graph,
    root_node_id: NodeT,
    len: NodeT,
    node_ids: Option<Vec<NodeT>>,
}

use std::string::ToString;
impl ToString for Circle {
    fn to_string(&self) -> String {
        let node_ids =
            if self.graph.has_node_types() && self.len() > 5 || self.graph.has_edge_types() {
                Some(self.get_circle_node_ids())
            } else {
                None
            };
        let show_node_type = if self.graph.has_node_types() {
            node_ids.as_ref().map_or(false, |node_ids| unsafe {
                !self
                    .graph
                    .has_unchecked_isomorphic_node_types_from_node_ids(node_ids)
            })
        } else {
            false
        };
        format!(
            concat!(
                "<p>",
                "Circle containing {number_of_nodes} nodes. ",
                "Specifically, the nodes involved in the circle are: {circle_nodes}.",
                "{node_types_counts}",
                "{edge_types_counts}",
                "</p>",
            ),
            number_of_nodes = to_human_readable_high_integer(self.len() as usize),
            circle_nodes = unsafe {
                get_unchecked_formatted_list(
                    &self
                        .get_circle_node_ids()
                        .into_iter()
                        .skip(1)
                        .map(|node_id| {
                            self.graph.get_unchecked_succinct_node_description(
                                node_id,
                                2,
                                show_node_type,
                            )
                        })
                        .collect::<Vec<String>>(),
                    Some(5),
                )
            },
            node_types_counts = if let Some(node_ids) = &node_ids {
                unsafe {
                    self.graph
                        .get_unchecked_node_type_id_counts_hashmap_from_node_ids(node_ids.as_ref())
                        .map_or_else(
                            |_| "".to_string(),
                            |count| {
                                if count.is_empty() {
                                    "".to_string()
                                } else {
                                    format!(
                                        " Its nodes have {}.",
                                        self.graph
                                            .get_unchecked_node_types_description_from_count(count)
                                    )
                                }
                            },
                        )
                }
            } else {
                "".to_string()
            },
            edge_types_counts = if let Some(node_ids) = &node_ids {
                unsafe {
                    self.graph
                        .get_unchecked_edge_type_id_counts_hashmap_from_node_ids(node_ids.as_ref())
                        .map_or_else(
                            |_| "".to_string(),
                            |count| {
                                if count.is_empty() {
                                    "".to_string()
                                } else {
                                    format!(
                                        " Its edges have {}.",
                                        self.graph
                                            .get_unchecked_edge_types_description_from_count(count)
                                    )
                                }
                            },
                        )
                }
            } else {
                "".to_string()
            }
        )
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.len.cmp(&other.len))
    }
}

impl Circle {
    /// Return new circle object created with the provided root and length.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph of reference of the circle.
    /// * `root_node_id`: NodeT - Minimum node ID of the circle.
    /// * `len`: NodeT - Precomputed circumference of the circle.
    ///
    pub(crate) fn new(graph: &Graph, root_node_id: NodeT, len: NodeT) -> Circle {
        Circle {
            graph: graph.clone(),
            root_node_id,
            len,
            node_ids: None,
        }
    }

    pub(crate) fn from_node_ids(graph: &Graph, node_ids: Vec<NodeT>) -> Circle {
        Circle {
            graph: graph.clone(),
            root_node_id: node_ids[0],
            len: node_ids.len() as NodeT,
            node_ids: Some(node_ids),
        }
    }

    /// Return the first node ID of the Circle.
    pub fn get_root_node_id(&self) -> NodeT {
        self.root_node_id
    }

    /// Return the first node name of the circle.
    pub fn get_root_node_name(&self) -> String {
        unsafe {
            self.graph
                .get_unchecked_node_name_from_node_id(self.root_node_id)
        }
    }

    /// Return length of the Circle.
    pub fn len(&self) -> NodeT {
        self.len
    }

    /// Return the node IDs of the nodes composing the Circle.
    pub fn get_circle_node_ids(&self) -> Vec<NodeT> {
        if let Some(node_ids) = &self.node_ids {
            node_ids.clone()
        } else {
            unsafe {
                self.graph
                    .get_circle_node_ids_from_root_node_id(self.root_node_id)
                    .unwrap()
            }
        }
    }

    /// Return the node names of the nodes composing the Circle.
    pub fn par_iter_circle_node_names(&self) -> impl IndexedParallelIterator<Item = String> + '_ {
        self.get_circle_node_ids()
            .into_par_iter()
            .map(move |node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Return the first `k` node IDs of the nodes composing the Circle.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_circle_node_ids(&self, k: usize) -> Vec<NodeT> {
        self.get_circle_node_ids().into_iter().take(k).collect()
    }

    /// Return the first `k` node names of the nodes composing the Circle.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_circle_node_names(&self, k: usize) -> Vec<String> {
        self.par_iter_circle_node_names().take(k).collect()
    }

    /// Return the node names of the nodes composing the Circle.
    pub fn get_circle_node_names(&self) -> Vec<String> {
        self.par_iter_circle_node_names().collect()
    }
}

impl Graph {
    /// Return node IDs in the Circle starting from the provided node ID.
    ///
    /// # Arguments
    /// `node_id`: NodeT - The root of the provided Circle.
    ///
    /// # Safety
    /// The node ID must be among the node IDs present in the graph, or the method will panic.
    /// Additionally, it must be the root node of a Circle.
    unsafe fn get_circle_node_ids_from_root_node_id(
        &self,
        root_node_id: NodeT,
    ) -> Option<Vec<NodeT>> {
        let mut circle_node_ids: Vec<NodeT> = vec![root_node_id];
        let mut node_id = root_node_id;
        let mut previous_node_id = node_id;
        'outer: loop {
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
            {
                if root_node_id > neighbour_node_id {
                    return None;
                }
                if self.get_chain_node_degree(neighbour_node_id) != 2 {
                    return None;
                }
                if neighbour_node_id != node_id
                    && neighbour_node_id != root_node_id
                    && neighbour_node_id != previous_node_id
                {
                    previous_node_id = node_id;
                    node_id = neighbour_node_id;
                    circle_node_ids.push(node_id);
                    continue 'outer;
                }
            }
            break;
        }
        Some(circle_node_ids)
    }

    /// Return option with number of the nodes in the circle,
    ///
    /// # Arguments
    /// `node_id`: NodeT - The root of the provided Circle.
    ///
    /// # Safety
    /// The node ID must be among the node IDs present in the graph, or the method will panic.
    /// Additionally, it must be the root node of a Circle.
    unsafe fn get_circle_number_of_nodes_from_root_node_id(
        &self,
        root_node_id: NodeT,
    ) -> Option<NodeT> {
        let mut number_of_nodes = 1;
        let mut node_id = root_node_id;
        let mut previous_node_id = node_id;
        'outer: loop {
            for neighbour_node_id in
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
            {
                if root_node_id > neighbour_node_id {
                    return None;
                }
                if self.get_chain_node_degree(neighbour_node_id) != 2 {
                    return None;
                }
                if neighbour_node_id != node_id
                    && neighbour_node_id != root_node_id
                    && neighbour_node_id != previous_node_id
                {
                    previous_node_id = node_id;
                    node_id = neighbour_node_id;
                    number_of_nodes += 1;
                    continue 'outer;
                }
            }
            break;
        }
        Some(number_of_nodes)
    }

    /// Return vector of Circles in the current graph instance.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_circle`: Option<NodeT> - Minimum size of the Circles.
    /// `compute_circle_nodes`: Option<bool> - Whether to pre-compute the Circle nodes.
    ///
    /// # Definitions
    /// A circle is a **component** where every node has degree 2 and each of its neighbours also have degree 2.
    /// The root of the circle is defined as the node with the smallest id in it.
    ///
    /// Here we use the same definition of degree as in the chains, so that we ignore selfloops and allow for
    /// multigraphs.
    pub fn get_circles(
        &self,
        minimum_number_of_nodes_per_circle: Option<NodeT>,
        compute_circle_nodes: Option<bool>,
    ) -> Result<Vec<Circle>> {
        self.must_be_undirected()?;
        let minimum_number_of_nodes_per_circle = minimum_number_of_nodes_per_circle.unwrap_or(5);
        let compute_circle_nodes = compute_circle_nodes.unwrap_or(false);

        Ok(self
            .par_iter_node_ids()
            .filter(|&node_id| unsafe {
                // quickly check if the given node has only two
                // neihgbours that also degree == 2
                // and that it's the local smallest (speed up)

                let mut node_degree = 0;
                let mut previous_node_id = node_id;
                for neighbour_node_id in
                    self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                {
                    // ignore selfloops and destinations already visited
                    if neighbour_node_id == node_id || neighbour_node_id == previous_node_id {
                        continue;
                    }
                    // ignore the node if it has a smaller neighbour
                    // because by definition also neighbour_node_id will be checked
                    // and thus we can reduce the number of nodes to check
                    if neighbour_node_id < node_id {
                        return false;
                    }
                    node_degree += 1;
                    // early stop
                    if node_degree > 2 || self.get_chain_node_degree(neighbour_node_id) != 2 {
                        return false;
                    }
                    previous_node_id = node_id;
                }
                true
            })
            .filter_map(|node_id| unsafe {
                if compute_circle_nodes {
                    self.get_circle_node_ids_from_root_node_id(node_id)
                        .and_then(|node_ids| {
                            if node_ids.len() as NodeT >= minimum_number_of_nodes_per_circle {
                                Some(Circle::from_node_ids(self, node_ids))
                            } else {
                                None
                            }
                        })
                } else {
                    self.get_circle_number_of_nodes_from_root_node_id(node_id)
                        .and_then(|number_of_nodes| {
                            if number_of_nodes >= minimum_number_of_nodes_per_circle {
                                Some(Circle::new(self, node_id, number_of_nodes))
                            } else {
                                None
                            }
                        })
                }
            })
            .collect::<Vec<Circle>>())
    }
}
