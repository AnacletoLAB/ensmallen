use super::*;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

#[derive(Hash, Clone, Debug)]
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
                "Specifically, the nodes involved in the star are: {}</p>",
            ),
            self.graph.get_name(),
            self.len(),
            self.get_root_node_name(),
            unsafe {get_unchecked_formatted_list(&self.get_star_node_names())}
        )
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

    /// Return the node names of the nodes composing the Star.
    pub fn get_star_node_names(&self) -> Vec<String> {
        self.get_star_node_ids()
            .into_par_iter()
            .map(|node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
            .collect()
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
        vec![root_node_id]
            .into_iter()
            .chain(
                self.iter_unchecked_neighbour_node_ids_from_source_node_id(root_node_id)
                    .filter(|&node_id| node_id != root_node_id),
            )
            .collect()
    }

    /// Return vector of Stars in the current graph instance.
    ///
    /// # Arguments
    /// `minimum_number_of_nodes_per_star`: Option<NodeT> - Minimum size of the Stars.
    /// `verbose`: Option<bool> - Whether to show the loading bars.
    pub fn get_stars(
        &self,
        minimum_number_of_nodes_per_star: Option<NodeT>,
        verbose: Option<bool>,
    ) -> Result<Vec<Star>> {
        self.must_be_undirected()?;
        let minimum_number_of_nodes_per_star = minimum_number_of_nodes_per_star.unwrap_or(10);
        let verbose = verbose.unwrap_or(true);
        let progress_bar = get_loading_bar(
            verbose,
            "Detecting nodes inside Stars",
            self.get_nodes_number() as usize,
        );
        Ok(self
            .par_iter_node_ids()
            .progress_with(progress_bar)
            .filter_map(|node_id| unsafe {
                let node_degree = self.get_unchecked_node_degree_from_node_id(node_id);
                if node_degree > minimum_number_of_nodes_per_star
                    && self
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                        .all(|neighbour_node_id| {
                            self.get_unchecked_node_degree_from_node_id(neighbour_node_id) == 1
                        })
                {
                    Some(Star::new(self, node_id, 1 + node_degree as NodeT))
                } else {
                    None
                }
            })
            .collect::<Vec<Star>>())
    }
}
