use super::*;
use log::info;
use rayon::prelude::*;
use std::cmp::Ordering;
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
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

    /// Return the first node ID of the DendriticTree.
    pub fn get_root_node_id(&self) -> NodeT {
        self.root_node_id
    }

    /// Return the first node name of the DendriticTree.
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
        if self.get_nodes_number() >= DENDRITIC_TREE_LEAF {
            return Err(format!(
                concat!(
                    "The current version of this algorithm ",
                    "does not support graphs with more than {} nodes."
                ),
                DENDRITIC_TREE_LEAF
            ));
        }
        let predecessors = ThreadDataRaceAware::new(vec![NODE_NOT_PRESENT; nodes_number]);

        // We initialize the initial frontier to the set of nodes with degree one.
        info!("Computing initial frontier.");
        let mut frontier: Vec<NodeT> = self
            .par_iter_node_ids()
            .filter_map(|node_id| unsafe {
                if self
                    .iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id)
                    .take(2)
                    == 1
                {
                    (*predecessors.value.get())[node_id as usize] = DENDRITIC_TREE_LEAF;
                    Some(node_id)
                } else {
                    None
                }
            })
            .collect::<Vec<NodeT>>();

        info!("Starting to explore the graph.");
        while !frontier.is_empty() {
            frontier = frontier
                .into_par_iter()
                .flat_map_iter(|node_id| unsafe {
                    // TODO!: The following line can be improved when the par iter is made
                    // generally available also for the elias-fano graphs.
                    self.iter_unchecked_unique_neighbour_node_ids_from_source_node_id(node_id)
                        .map(move |neighbour_node_id| (neighbour_node_id, node_id))
                })
                .filter_map(|(neighbour_node_id, node_id)| unsafe {
                    // If this node was not yet explored
                    if (*predecessors.value.get())[neighbour_node_id as usize] == NODE_NOT_PRESENT {
                        // Check if this new node is a validate dentritic leaf, that is, it is a node
                        // with all neighbours MINUS ONE equal to dentritic leafs. All the nodes we explore
                        // are dentritic leafs, therefore this means to check how many nodes are not explored.
                        // We only want to check if the new node is with one or more neighbours that are not visited
                        // and we do not need to explore all the other neighbours.
                        let unexplored_neighbours = self
                            .iter_unchecked_unique_neighbour_node_ids_from_source_node_id(
                                neighbour_node_id,
                            )
                            .filter(|&farther_node_id| {
                                (*predecessors.value.get())[farther_node_id as usize]
                                    == NODE_NOT_PRESENT
                            })
                            .take(2)
                            .count();
                        if unexplored_neighbours == 1 {
                            // Set the neighbouring node as a dentritic tree leaf.
                            (*predecessors.value.get())[neighbour_node_id as usize] =
                                DENDRITIC_TREE_LEAF;
                            // add the node to the nodes to explore
                            Some(neighbour_node_id)
                        } else {
                            // Set the neighbouring node as a dentritic tree root.
                            (*predecessors.value.get())[neighbour_node_id as usize] =
                                neighbour_node_id;
                            None
                        }
                    } else {
                        // If the neighbour is described as a dentritic leaf that has not
                        // yet a parent, its parent node will be the current node.
                        if (*predecessors.value.get())[neighbour_node_id as usize]
                            == DENDRITIC_TREE_LEAF
                        {
                            (*predecessors.value.get())[neighbour_node_id as usize] = node_id;
                        }
                        None
                    }
                })
                .collect::<Vec<NodeT>>();
        }

        let predecessors: Vec<NodeT> = predecessors.value.into_inner();
        info!("Searching root nodes.");
        let roots = predecessors
            .par_iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, node_id)| {
                if node_id == i as NodeT {
                    Some(node_id)
                } else {
                    None
                }
            })
            .collect::<Vec<NodeT>>();
        info!("Detected {} dendritic trees.", roots.len());
        Ok(roots
            .into_par_iter()
            .map(|root_node_id| {
                let mut leaf_nodes: Vec<NodeT> = Vec::new();
                let mut depth: NodeT = 0;
                let mut stack: Vec<NodeT> = predecessors
                    .iter()
                    .cloned()
                    .enumerate()
                    .filter_map(|(i, node_id)| {
                        // If the current node has as parent the root node
                        // but isn't the root node itself, which is represented
                        // as a selfloop.
                        if root_node_id == node_id && i as NodeT != root_node_id {
                            Some(i as NodeT)
                        } else {
                            None
                        }
                    })
                    .collect();
                while !stack.is_empty() {
                    depth += 1;
                    leaf_nodes.extend_from_slice(&stack);
                    stack = predecessors
                        .iter()
                        .enumerate()
                        .filter_map(move |(i, node_id)| {
                            if stack.contains(node_id) {
                                Some(i as NodeT)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<NodeT>>();
                }
                DendriticTree::from_node_ids(&self, root_node_id, depth, leaf_nodes)
            })
            .collect::<Vec<DendriticTree>>())
    }
}
