use super::*;
use log::info;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
pub const DENDRITIC_TREE_LEAF: NodeT = NodeT::MAX - 1;

#[derive(Hash, Clone, Debug, PartialEq)]
pub struct DendriticTree {
    graph: Graph,
    root_node_id: NodeT,
    len: NodeT,
    depth: NodeT,
    node_ids: Vec<NodeT>,
}

use std::string::ToString;
impl ToString for DendriticTree {
    fn to_string(&self) -> String {
        format!(
            concat!(
                "<p>Dendritic Tree starting from the root node {root_node_description}, ",
                "and {other_nodes_description}.</p>"
            ),
            root_node_description = unsafe {
                self.graph
                    .get_unchecked_succinct_node_description(self.get_root_node_id(), 1)
            },
            other_nodes_description = match self.len() {
                0 => unreachable!("It does not make sense to have an empty dendritic tree."),
                1 => format!("containing a single other node, {}", unsafe {
                    self.graph
                        .get_unchecked_succinct_node_description(self.node_ids[0], 1)
                }),
                nodes_number => format!(
                    concat!("containing {} nodes, with a maximal depth of {}, which are {}"),
                    to_human_readable_high_integer(nodes_number as usize),
                    to_human_readable_high_integer(self.depth as usize),
                    unsafe {
                        get_unchecked_formatted_list(
                            &self
                                .get_dentritic_trees_node_ids()
                                .into_iter()
                                .map(|node_id| {
                                    self.graph
                                        .get_unchecked_succinct_node_description(node_id, 2)
                                })
                                .collect::<Vec<String>>(),
                            Some(5),
                        )
                    }
                ),
            }
        )
    }
}

impl PartialOrd for DendriticTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.len.cmp(&other.len))
    }
}

impl DendriticTree {
    pub(crate) fn from_node_ids(
        graph: &Graph,
        root_node_id: NodeT,
        depth: NodeT,
        node_ids: Vec<NodeT>,
    ) -> DendriticTree {
        DendriticTree {
            graph: graph.clone(),
            root_node_id,
            len: node_ids.len() as NodeT,
            depth,
            node_ids,
        }
    }

    /// Return the root node ID of the dendritic tree.
    pub fn get_root_node_id(&self) -> NodeT {
        self.root_node_id
    }

    /// Return the depth of the dentritic tree.
    pub fn get_depth(&self) -> NodeT {
        self.depth
    }

    /// Return the root node name of the DendriticTree.
    pub fn get_root_node_name(&self) -> String {
        unsafe {
            self.graph
                .get_unchecked_node_name_from_node_id(self.root_node_id)
        }
    }

    /// Return length of the DendriticTree.
    pub fn len(&self) -> NodeT {
        self.len
    }

    /// Return the node IDs of the nodes composing the DendriticTree.
    pub fn get_dentritic_trees_node_ids(&self) -> Vec<NodeT> {
        self.node_ids.clone()
    }

    /// Return the node names of the nodes composing the DendriticTree.
    pub fn par_iter_dentritic_trees_node_names(
        &self,
    ) -> impl IndexedParallelIterator<Item = String> + '_ {
        self.get_dentritic_trees_node_ids()
            .into_par_iter()
            .map(move |node_id| unsafe { self.graph.get_unchecked_node_name_from_node_id(node_id) })
    }

    /// Return the first `k` node IDs of the nodes composing the DendriticTree.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_dentritic_trees_node_ids(&self, k: usize) -> Vec<NodeT> {
        self.get_dentritic_trees_node_ids()
            .into_iter()
            .take(k)
            .collect()
    }

    /// Return the first `k` node names of the nodes composing the DendriticTree.
    ///
    /// # Arguments
    /// `k`: usize - The number of terms to return.
    pub fn get_first_k_dentritic_trees_node_names(&self, k: usize) -> Vec<String> {
        self.par_iter_dentritic_trees_node_names().take(k).collect()
    }

    /// Return the node names of the nodes composing the DendriticTree.
    pub fn get_dentritic_trees_node_names(&self) -> Vec<String> {
        self.par_iter_dentritic_trees_node_names().collect()
    }
}

impl Graph {
    /// Returns vector of detected dentritic trees.
    pub fn get_dendritic_trees(&self) -> Result<Vec<DendriticTree>> {
        self.must_be_undirected()?;
        let nodes_number = self.get_nodes_number() as usize;
        let leaf_nodes = ThreadDataRaceAware::new(vec![false; nodes_number]);

        // We initialize the initial frontier to the set of nodes with degree one.
        info!("Computing initial frontier.");
        let mut frontier: Vec<NodeT> = self
            .par_iter_node_ids()
            .filter_map(|node_id| unsafe {
                if self
                    .iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id)
                    .take(2)
                    .count()
                    == 1
                {
                    (*leaf_nodes.value.get())[node_id as usize] = true;
                    Some(node_id)
                } else {
                    None
                }
            })
            .collect::<Vec<NodeT>>();

        let expanded_frontier = AtomicBool::new(true);
        info!("Starting to explore the graph.");
        while expanded_frontier.load(Ordering::Relaxed) {
            expanded_frontier.store(false, Ordering::Relaxed);
            frontier = frontier
                .into_par_iter()
                .flat_map_iter(|node_id| unsafe {
                    // If this is a candidate root, we pass it without further exploring
                    // its neighbouring nodes in order to not expand further the
                    let iterator: Box<dyn Iterator<Item = NodeT>> =
                        if (*leaf_nodes.value.get())[node_id as usize] {
                            Box::new(
                                self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
                                    node_id,
                                )
                                .filter(|&neighbour_node_id| {
                                    !(*leaf_nodes.value.get())[neighbour_node_id as usize]
                                })
                            )
                        } else {
                            Box::new(vec![node_id].into_iter())
                        };
                    iterator
                })
                .filter_map(|neighbour_node_id| unsafe {
                    // We retrieve the number of neighbours of the node that is NOT a
                    // dentritic leaf, and if the number is exactly equal to one
                    // we can mark this new node also as a dentritic leaf.
                    let unexplored_neighbours = self
                        .iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
                            neighbour_node_id,
                        )
                        .filter(|&farther_node_id| {
                            !(*leaf_nodes.value.get())[farther_node_id as usize]
                        })
                        .take(2)
                        .count();
                    if unexplored_neighbours == 1 {
                        // Set the neighbouring node as a dentritic tree leaf.
                        (*leaf_nodes.value.get())[neighbour_node_id as usize] = true;
                        expanded_frontier.store(true, Ordering::Relaxed);
                    }
                    Some(neighbour_node_id)
                })
                .collect::<Vec<NodeT>>();
        }

        info!("Searching root nodes.");
        // The nodes remaining in the frontier at convergence are root
        // nodes, but they may be appearing multiple times. We need
        // to make these values unique.
        frontier.par_sort_unstable();
        frontier.dedup();

        info!("Detected {} dendritic trees.", frontier.len());
        Ok(frontier
            .into_par_iter()
            .map(|root_node_id| {
                let mut tree_nodes: Vec<NodeT> = Vec::new();
                let mut depth: NodeT = 0;
                let mut stack: Vec<NodeT> = vec![root_node_id];
                while !stack.is_empty() {
                    depth += 1;
                    stack = stack
                        .iter()
                        .flat_map(|&node_id| unsafe {
                            self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
                                node_id,
                            )
                            .filter(|&neighbour_node_id| {
                                (*leaf_nodes.value.get())[neighbour_node_id as usize]
                            })
                        })
                        .map(|neighbour_node_id| unsafe {
                            (*leaf_nodes.value.get())[neighbour_node_id as usize] = false;
                            neighbour_node_id
                        })
                        .collect::<Vec<NodeT>>();
                    tree_nodes.extend_from_slice(&stack);
                }
                depth -= 1;
                DendriticTree::from_node_ids(&self, root_node_id, depth, tree_nodes)
            })
            .collect::<Vec<DendriticTree>>())
    }
}
