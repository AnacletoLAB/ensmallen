use atomic_float::AtomicF32;
use graph::{Graph, NodeT};
use rayon::prelude::*;
use std::sync::atomic::Ordering;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Visited {
    Unvisited = 0,
    VisitedByFirstNode = 1,
    VisitedBySecondNode = 2,
}

impl std::fmt::Display for Visited {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct DAGResnik {
    /// The transposed DAG to be used to search preprocessors.
    transposed_dag: Option<Graph>,
    /// Frequencies of the nodes.
    node_frequencies: Vec<f32>,
}

impl DAGResnik {
    /// Return new instance of DAG-based Resnik for similarity computation.
    pub fn new() -> Self {
        Self {
            transposed_dag: None,
            node_frequencies: Vec::new(),
        }
    }

    fn must_be_trained(&self) -> Result<(), String> {
        if self.transposed_dag.is_none() {
            return Err(concat!(
                "This model has not been trained yet. ",
                "You should call the `.fit` method first."
            )
            .to_string());
        }
        Ok(())
    }

    /// Returns the node frequencies of the model.
    pub fn get_node_frequencies(&self) -> Result<Vec<f32>, String> {
        self.must_be_trained()
            .map(|_| self.node_frequencies.clone())
    }

    fn validate_features(
        &self,
        graph: &Graph,
        node_frequencies: Option<&[f32]>,
    ) -> Result<(), String> {
        graph.must_be_directed_acyclic()?;
        if let Some(node_frequencies) = node_frequencies.as_ref() {
            if node_frequencies.len() == 0 {
                return Err(concat!(
                    "The provided frequencies dimensions is zero. ",
                    "The number of node frequenciess should be a strictly positive value."
                )
                .to_string());
            }

            if node_frequencies.len() != graph.get_number_of_nodes() as usize {
                return Err(format!(
                    concat!(
                        "The provided node frequenciess have size {}, but the expected size ",
                        "based on the provided graph {}."
                    ),
                    node_frequencies.len(),
                    graph.get_number_of_nodes()
                ));
            }
        }

        Ok(())
    }

    /// Fit the model with the provided graph and
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be learned.
    /// * `node_frequencies`: Option<&[f32]> - Optional vector of node frequencies.
    pub fn fit(&mut self, graph: &Graph, node_frequencies: Option<&[f32]>) -> Result<(), String> {
        self.validate_features(graph, node_frequencies)?;
        let mut transposed_graph = graph.to_transposed();
        transposed_graph.enable(
            Some(graph.has_sources_tradeoff_enabled()),
            Some(graph.has_destinations_tradeoff_enabled()),
            Some(graph.has_cumulative_node_degrees_tradeoff_enabled()),
            Some(graph.has_reciprocal_sqrt_degrees_tradeoff_enabled()),
        )?;
        if let Some(node_frequencies) = node_frequencies {
            self.node_frequencies = node_frequencies.into();
        } else {
            let number_of_nodes = graph.get_number_of_nodes() as usize;
            let node_frequencies = unsafe {
                std::mem::transmute::<Vec<f32>, Vec<AtomicF32>>(vec![0.0; number_of_nodes])
            };
            let mut frontier = graph
                .par_iter_trap_node_ids()
                .map(|leaf_node_id| {
                    node_frequencies[leaf_node_id as usize].fetch_add(1.0, Ordering::Relaxed);
                    leaf_node_id
                })
                .flat_map_iter(|leaf_node_id| unsafe {
                    transposed_graph.iter_unchecked_neighbour_node_ids_from_source_node_id(leaf_node_id)
                })
                .collect::<Vec<NodeT>>();
            let mut depth = 1.0;
            while !frontier.is_empty() {
                depth += 1.0;
                frontier = frontier
                    .into_par_iter()
                    .map(|leaf_node_id| {
                        node_frequencies[leaf_node_id as usize].fetch_add(depth, Ordering::Relaxed);
                        leaf_node_id
                    })
                    .flat_map_iter(|leaf_node_id| unsafe {
                        transposed_graph
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(leaf_node_id)
                    })
                    .collect::<Vec<NodeT>>();
            }
            let mut node_frequencies =
                unsafe { std::mem::transmute::<Vec<AtomicF32>, Vec<f32>>(node_frequencies) };
            let total = node_frequencies.par_iter().sum::<f32>();
            node_frequencies.par_iter_mut().for_each(|node_frequency| {
                *node_frequency = -(*node_frequency / total).ln();
            });
            self.node_frequencies = node_frequencies;
        }
        self.transposed_dag = Some(transposed_graph);
        Ok(())
    }

    /// Return the similarity between the two provided nodes.
    ///
    /// # Arguments
    /// * `first_node_id`: NodeT - The first node for which to compute the similarity.
    /// * `second_node_id`: NodeT - The second node for which to compute the similarity.
    pub fn get_similarity_from_node_id(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> Result<f32, String> {
        self.must_be_trained()?;
        if let Some(transposed_dag) = self.transposed_dag.as_ref() {
            let mut visited: Vec<Visited> =
                vec![Visited::Unvisited; transposed_dag.get_number_of_nodes() as usize];
                visited[first_node_id as usize] = Visited::VisitedByFirstNode;
                visited[second_node_id as usize] = Visited::VisitedBySecondNode;
            let mut frontier = vec![first_node_id, second_node_id];
            while !frontier.is_empty() {
                let mut new_frontier = Vec::new();
                for leaf_node in frontier {
                    for parent_node_id in unsafe {
                        transposed_dag
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(leaf_node)
                    } {
                        match (
                            &visited[leaf_node as usize],
                            &visited[parent_node_id as usize],
                        ) {
                            (Visited::VisitedByFirstNode, Visited::VisitedByFirstNode) => {}
                            (Visited::VisitedBySecondNode, Visited::VisitedBySecondNode) => {}
                            (Visited::VisitedByFirstNode, Visited::VisitedBySecondNode) => {
                                return Ok(self.node_frequencies[parent_node_id as usize]);
                            }
                            (Visited::VisitedBySecondNode, Visited::VisitedByFirstNode) => {
                                return Ok(self.node_frequencies[parent_node_id as usize]);
                            }
                            (label, Visited::Unvisited) => {
                                visited[parent_node_id as usize] = *label;
                                new_frontier.push(parent_node_id);
                            }
                            (x, y) => unreachable!(
                                concat!(
                                    "The case with the leaf node in state {} and ",
                                    "the parent node in state {} should not be possible."
                                ),
                                x, y
                            ),
                        }
                    }
                }
                frontier = new_frontier;
            }
        }
        Err(format!(
            concat!(
                "The provided two nodes {} and {} do not have a shared ",
                "parent node. Perhaps, the provided DAG has multiple root nodes ",
                "and these two nodes are in different root portions of the DAG. ",
                "Another analogous explanation is that the two nodes may be in ",
                "different connected components."
            ),
            first_node_id, second_node_id
        ))
    }

    /// Writes the predicted similarities on the provided memory area.
    ///
    /// # Arguments
    /// * `similarities`: &mut [f32] - Area where to write the similarities.
    /// * `graph`: &Graph - The graph whose edges are to be computed.
    pub fn get_similarities_from_graph(
        &self,
        similarities: &mut [f32],
        graph: &Graph,
    ) -> Result<(), String> {
        self.must_be_trained()?;

        if similarities.len() != graph.get_number_of_directed_edges() as usize {
            return Err(format!(
                concat!(
                    "The provided similarities slice has size `{}` ",
                    "but it was expected to have the same ",
                    "size of the number of the directed edges in the graph `{}`."
                ),
                similarities.len(),
                graph.get_number_of_directed_edges()
            ));
        }

        similarities
            .par_iter_mut()
            .zip(graph.par_iter_directed_edge_node_ids())
            .map(|(similarity, (_, src, dst))| {
                self.get_similarity_from_node_id(src, dst)
                    .map(|nodes_similarity| {
                        *similarity = nodes_similarity;
                        ()
                    })
            })
            .collect::<Result<(), String>>()
    }
}
