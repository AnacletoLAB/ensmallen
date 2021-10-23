use super::*;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

#[derive(Hash, Clone, Debug)]
pub struct Circle {
    graph: Graph,
    root_node_id: NodeT,
    len: NodeT,
    node_ids: Option<Vec<NodeT>>,
}

use std::string::ToString;
impl ToString for Circle {
    fn to_string(&self) -> String {
        format!(
            concat!(
                "<p>This circle of nodes from the graph {} contains {} nodes. ",
                "Specifically, the nodes involved in the circle are: {}</p>",
            ),
            self.graph.get_name(),
            self.len(),
            unsafe {get_unchecked_formatted_list(&self.get_circle_node_names())}
        )
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
    pub fn get_circle_node_names(&self) -> Vec<String> {
        self.get_circle_node_ids()
            .into_par_iter()
            .map(|node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
            .collect()
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
                if neighbour_node_id != node_id
                    && neighbour_node_id != root_node_id
                    && neighbour_node_id != previous_node_id
                    && self.get_unchecked_node_degree_from_node_id(neighbour_node_id) == 2
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

    /// Return vector of Circles in the current graph instance.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_circle`: Option<NodeT> - Minimum size of the Circles.
    /// `compute_circle_nodes`: Option<bool> - Whether to pre-compute the Circle nodes.
    /// `verbose`: Option<bool> - Whether to show the loading bars.
    pub fn get_circles(
        &self,
        minimum_number_of_nodes_per_circle: Option<NodeT>,
        compute_circle_nodes: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Vec<Circle>> {
        self.must_be_undirected()?;
        let minimum_number_of_nodes_per_circle = minimum_number_of_nodes_per_circle.unwrap_or(10);
        let verbose = verbose.unwrap_or(true);
        let compute_circle_nodes = compute_circle_nodes.unwrap_or(false);
        let progress_bar = get_loading_bar(
            verbose,
            "Detecting nodes inside Circles",
            self.get_nodes_number() as usize,
        );
        Ok(self
            .par_iter_node_ids()
            .progress_with(progress_bar)
            .filter(|&node_id| unsafe {
                self.get_unchecked_node_degree_from_node_id(node_id) == 2
                    && self
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .all(|neighbour_node_id| {
                            neighbour_node_id > node_id
                                && self.get_unchecked_node_degree_from_node_id(neighbour_node_id)
                                    == 2
                        })
            })
            .filter_map(|node_id| unsafe {
                if let Some(node_ids) = self.get_circle_node_ids_from_root_node_id(node_id) {
                    if node_ids.len() as NodeT >= minimum_number_of_nodes_per_circle {
                        Some(if compute_circle_nodes {
                            Circle::from_node_ids(self, node_ids)
                        } else {
                            Circle::new(self, node_id, node_ids.len() as NodeT)
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<Circle>>())
    }
}
