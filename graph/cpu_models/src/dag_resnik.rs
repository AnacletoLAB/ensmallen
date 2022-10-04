use graph::{Graph, NodeT};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use num_traits::{Coerced, Float};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

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
pub struct DAGResnik<F: Float> {
    /// The transposed DAG to be used to search preprocessors.
    transposed_dag: Option<Graph>,
    /// Frequencies of the nodes.
    node_frequencies: Vec<F>,
    /// Whether to show loading bars when computing pairwise similarity.
    verbose: bool,
}

impl<F: Float + Send + Sync> DAGResnik<F>
where
    u32: Coerced<F>,
{
    /// Return new instance of DAG-based Resnik for similarity computation.
    ///
    /// # Arguments
    /// * `verbose`: bool - Whether to show loading bars when computing pairwise similarity.
    pub fn new(verbose: Option<bool>) -> Self {
        Self {
            transposed_dag: None,
            node_frequencies: Vec::new(),
            verbose: verbose.unwrap_or(true),
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
    pub fn get_number_of_nodes(&self) -> Result<NodeT, String> {
        self.must_be_trained()?;
        if let Some(transposed_dag) = self.transposed_dag.as_ref() {
            return Ok(transposed_dag.get_number_of_nodes());
        }
        unreachable!("")
    }

    /// Returns the number of nodes in the current graph.
    pub fn get_node_frequencies(&self) -> Result<Vec<F>, String> {
        self.must_be_trained()
            .map(|_| self.node_frequencies.clone())
    }

    fn validate_features(
        &self,
        graph: &Graph,
        node_counts: Option<&HashMap<String, u32>>,
        node_frequencies: Option<&[F]>,
    ) -> Result<(), String> {
        graph.must_be_directed_acyclic()?;
        if let Some(node_counts) = node_counts.as_ref() {
            node_counts
                .par_iter()
                .map(|(node_name, _)| graph.get_node_id_from_node_name(node_name).map(|_| ()))
                .collect::<Result<(), String>>()?;
        }
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
    /// * `node_counts`: Option<HashMap<String, u32>> - Hashmap of node counts. These counts should represent how many times a given node appears in a set.
    /// * `node_frequencies`: Option<&[F]> - Optional vector of node frequencies to be used WITHOUT crawling upwards the DAG.
    pub fn fit(
        &mut self,
        graph: &Graph,
        node_counts: Option<&HashMap<String, u32>>,
        node_frequencies: Option<&[F]>,
    ) -> Result<(), String> {
        self.validate_features(graph, node_counts, node_frequencies)?;
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
            let mut node_counts = if let Some(node_counts) = node_counts {
                graph
                    .par_iter_node_names()
                    .map(|node_name| *node_counts.get(&node_name).unwrap_or(&0))
                    .collect::<Vec<u32>>()
            } else {
                vec![1; graph.get_number_of_nodes() as usize]
            };
            let visited_by_all_child = unsafe {
                std::mem::transmute::<Vec<bool>, Vec<AtomicBool>>(vec![
                    false;
                    graph.get_number_of_nodes()
                        as usize
                ])
            };
            let mut frontier = graph
                .par_iter_trap_node_ids()
                .flat_map_iter(|leaf_node_id| unsafe {
                    visited_by_all_child[leaf_node_id as usize].store(true, Ordering::SeqCst);
                    transposed_graph
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(leaf_node_id)
                })
                .collect::<Vec<NodeT>>();
            while !frontier.is_empty() {
                frontier = frontier
                    .into_iter()
                    .flat_map(|node_id| unsafe {
                        // If any of the children nodes of this node
                        // were not visited
                        if graph
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .any(|child_node_id| {
                                !visited_by_all_child[child_node_id as usize].load(Ordering::SeqCst)
                            })
                        {
                            // We cannot visit it yet, so we will visit it in the
                            // future.
                            return vec![node_id];
                        }
                        // Otherwise we mark this node as visited
                        visited_by_all_child[node_id as usize].store(true, Ordering::SeqCst);
                        // And we proceed to compute its value.
                        node_counts[node_id as usize] = graph
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .map(|child_node_id| node_counts[child_node_id as usize])
                            .sum::<u32>();
                        // And we return its parents as the next nodes to be visited.
                        transposed_graph
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .collect::<Vec<NodeT>>()
                    })
                    .collect::<Vec<NodeT>>();
            }
            let root_node_count: F = node_counts
                .par_iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
                .coerce_into();
            self.node_frequencies = node_counts
                .into_par_iter()
                .map(|node_count| -(node_count.coerce_into() / root_node_count).ln())
                .collect::<Vec<F>>();
        }
        self.transposed_dag = Some(transposed_graph);
        Ok(())
    }

    /// Return the similarity between the two provided node ids.
    ///
    /// # Arguments
    /// * `first_node_id`: NodeT - The first node for which to compute the similarity.
    /// * `second_node_id`: NodeT - The second node for which to compute the similarity.
    pub fn get_similarity_from_node_id(
        &self,
        first_node_id: NodeT,
        second_node_id: NodeT,
    ) -> Result<F, String> {
        self.must_be_trained()?;
        if let Some(transposed_dag) = self.transposed_dag.as_ref() {
            if first_node_id == second_node_id {
                return Ok(self.node_frequencies[first_node_id as usize]);
            }
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
                            (Visited::Unvisited, _) => unreachable!(
                                "The case with an unvisited leaf node should not be possible."
                            ),
                        }
                    }
                }
                frontier = new_frontier;
            }
        }

        let root_node_names = self
            .transposed_dag
            .as_ref()
            .map_or_else(|| Vec::new(), |graph| graph.get_root_node_names());
        Err(format!(
            concat!(
                "The provided two nodes {} and {} do not have a shared ",
                "parent node. Perhaps, the provided DAG has multiple root nodes ",
                "and these two nodes are in different root portions of the DAG. ",
                "Another analogous explanation is that the two nodes may be in ",
                "different connected components. ",
                "Do be advised that this DAG has {} root nodes, with the first ones ",
                "being {:?}."
            ),
            first_node_id,
            second_node_id,
            root_node_names.len(),
            &root_node_names[0..5]
        ))
    }

    /// Return Resnik similarities.
    ///
    /// # Arguments
    /// * `iterator`: Parallel iterator of node IDs to compute the similarities for.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    ///
    /// # Implementation details
    /// If the DAG has multiple roots, values that do not have
    /// the same head root are filtered out.
    fn iter_similarities_from_node_ids_iterator<'a, I>(
        &'a self,
        iterator: I,
        minimum_similarity: Option<F>,
    ) -> Result<impl ParallelIterator<Item = (Vec<NodeT>, F)> + 'a, String>
    where
        I: ParallelIterator<Item = (NodeT, NodeT)> + 'a,
    {
        self.must_be_trained()?;
        let minimum_similarity = minimum_similarity.unwrap_or(F::zero());
        Ok(iterator.filter_map(move |(src, dst)| {
            if let Ok(similarity) = self.get_similarity_from_node_id(src, dst) {
                if similarity > minimum_similarity {
                    Some((vec![src, dst], similarity))
                } else {
                    None
                }
            } else {
                None
            }
        }))
    }

    /// Return the similarity between the two provided node ids.
    ///
    /// # Arguments
    /// * `first_node_ids`: Vec<NodeT> - The first node ids for which to compute the similarity.
    /// * `second_node_ids`: Vec<NodeT> - The second node ids for which to compute the similarity.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_similarity_from_node_ids(
        &self,
        first_node_ids: Vec<NodeT>,
        second_node_ids: Vec<NodeT>,
        minimum_similarity: Option<F>,
    ) -> Result<(Vec<Vec<NodeT>>, Vec<F>), String> {
        Ok(self
            .iter_similarities_from_node_ids_iterator(
                (first_node_ids, second_node_ids).into_par_iter(),
                minimum_similarity,
            )?
            .unzip())
    }

    /// Return the similarity between the two provided node names.
    ///
    /// # Arguments
    /// * `first_node_name`: &str - The first node name for which to compute the similarity.
    /// * `second_node_name`: &str - The second node name for which to compute the similarity.
    pub fn get_similarity_from_node_name(
        &self,
        first_node_name: &str,
        second_node_name: &str,
    ) -> Result<F, String> {
        self.must_be_trained()?;
        if let Some(transposed_dag) = self.transposed_dag.as_ref() {
            return self.get_similarity_from_node_id(
                transposed_dag.get_node_id_from_node_name(first_node_name)?,
                transposed_dag.get_node_id_from_node_name(second_node_name)?,
            );
        }
        unreachable!("")
    }

    /// Return Resnik similarities.
    ///
    /// # Arguments
    /// * `iterator`: Parallel iterator of node names to compute the similarities for.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    ///
    /// # Implementation details
    /// If the DAG has multiple roots, values that do not have
    /// the same head root are filtered out.
    fn iter_similarities_from_node_names_iterator<'a, I>(
        &'a self,
        iterator: I,
        minimum_similarity: Option<F>,
    ) -> Result<impl ParallelIterator<Item = ([String; 2], F)> + 'a, String>
    where
        I: ParallelIterator<Item = (String, String)> + 'a,
    {
        self.must_be_trained()?;
        let minimum_similarity = minimum_similarity.unwrap_or(F::zero());
        Ok(iterator.filter_map(move |(src, dst)| {
            if let Ok(similarity) = self.get_similarity_from_node_name(&src, &dst) {
                if similarity > minimum_similarity {
                    Some(([src, dst], similarity))
                } else {
                    None
                }
            } else {
                None
            }
        }))
    }

    /// Return the similarity between the two provided node names.
    ///
    /// # Arguments
    /// * `first_node_names`: Vec<String> - The first node names for which to compute the similarity.
    /// * `second_node_names`: Vec<String> - The second node names for which to compute the similarity.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_similarity_from_node_names(
        &self,
        first_node_names: Vec<String>,
        second_node_names: Vec<String>,
        minimum_similarity: Option<F>,
    ) -> Result<(Vec<[String; 2]>, Vec<F>), String> {
        Ok(self
            .iter_similarities_from_node_names_iterator(
                (first_node_names, second_node_names).into_par_iter(),
                minimum_similarity,
            )?
            .unzip())
    }

    /// Return the similarity between the two provided node name prefixes.
    ///
    /// # Arguments
    /// * `first_node_prefixes`: Vec<&str> - The first node prefixes for which to compute the similarity.
    /// * `second_node_prefixes`: Vec<&str> - The second node prefixes for which to compute the similarity.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_similarity_from_node_prefixes(
        &self,
        first_node_prefixes: Vec<&str>,
        second_node_prefixes: Vec<&str>,
        minimum_similarity: Option<F>,
    ) -> Result<(Vec<[String; 2]>, Vec<F>), String> {
        self.must_be_trained()?;
        let sources = self
            .transposed_dag
            .as_ref()
            .map(|graph| graph.par_iter_node_names_from_node_curie_prefixes(&first_node_prefixes))
            .unwrap();
        Ok(self
            .iter_similarities_from_node_names_iterator(
                sources.flat_map(|src| {
                    self.transposed_dag
                        .as_ref()
                        .map(|graph| {
                            graph.par_iter_node_names_from_node_curie_prefixes(&second_node_prefixes)
                        })
                        .unwrap()
                        .map(move |dst| (src.clone(), dst))
                }),
                minimum_similarity,
            )?
            .unzip())
    }

    /// Writes the pairwise similarities on the provided memory area.
    ///
    /// # Arguments
    /// * `similarities`: &mut [F] - Area where to write the pairwise similarities.
    pub fn get_pairwise_similarities(&self, similarities: &mut [F]) -> Result<(), String> {
        self.must_be_trained()?;
        let nodes_number = self.get_number_of_nodes()? as usize;
        if similarities.len() != nodes_number * nodes_number {
            return Err(format!(
                concat!(
                    "The provided similarities slice has size `{}` ",
                    "but it was expected to have the same ",
                    "size of the number of the squared number of nodes in the graph `{}`."
                ),
                similarities.len(),
                nodes_number * nodes_number
            ));
        }

        let progress_bar = if self.verbose {
            let pb = ProgressBar::new(nodes_number as u64);
            pb.set_style(ProgressStyle::default_bar().template(concat!(
                "Computing pairwise Resnik ",
                "{spinner:.green} [{elapsed_precise}] ",
                "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
            )).unwrap());
            pb
        } else {
            ProgressBar::hidden()
        };

        similarities
            .par_chunks_mut(nodes_number)
            .progress_with(progress_bar)
            .enumerate()
            .for_each(|(src, row)| {
                row.iter_mut().enumerate().for_each(|(dst, similarity)| {
                    *similarity = self
                        .get_similarity_from_node_id(src as NodeT, dst as NodeT)
                        .unwrap_or(F::infinity())
                });
            });
        Ok(())
    }

    /// Writes the similarities on the provided memory area.
    ///
    /// # Arguments
    /// * `graph`: &Graph - The graph whose edges are to be computed.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    ///
    /// # Implementation details
    /// If the DAG has multiple roots, values that do not have
    /// the same head root are filtered out.
    pub fn get_similarities_from_graph(
        &self,
        graph: &Graph,
        minimum_similarity: Option<F>,
    ) -> Result<(Vec<Vec<NodeT>>, Vec<F>), String> {
        self.must_be_trained()?;
        Ok(self
            .iter_similarities_from_node_ids_iterator(
                graph
                    .par_iter_directed_edge_node_ids()
                    .map(|(_, src, dst)| (src, dst)),
                minimum_similarity,
            )?
            .unzip())
    }
}
