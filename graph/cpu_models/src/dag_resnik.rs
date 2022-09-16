use file_progress::{FileProgress, FileProgressIterator, MarkdownFileProgress};
use graph::{Graph, NodeT, ThreadDataRaceAware};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use num_traits::{Coerced, Float, IntoAtomic};
use parallel_frontier::Frontier;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub struct DAGResnik<F: Float> {
    /// The transposed DAG to be used to search preprocessors.
    undirected_dag: Option<Graph>,
    /// Root node IDs of the DAG
    root_node_ids: Vec<NodeT>,
    /// Frequencies of the nodes.
    information_contents: Vec<F>,
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
            undirected_dag: None,
            root_node_ids: Vec::new(),
            information_contents: Vec::new(),
            verbose: verbose.unwrap_or(true),
        }
    }

    fn must_be_trained(&self) -> Result<(), String> {
        if self.undirected_dag.is_none() {
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
        if let Some(undirected_dag) = self.undirected_dag.as_ref() {
            return Ok(undirected_dag.get_number_of_nodes());
        }
        unreachable!("")
    }

    /// Returns the number of nodes in the current graph.
    pub fn get_information_contents(&self) -> Result<Vec<F>, String> {
        self.must_be_trained()
            .map(|_| self.information_contents.clone())
    }

    fn validate_features(
        &self,
        graph: &Graph,
        node_counts: Option<&HashMap<String, u32>>,
        information_contents: Option<&[F]>,
    ) -> Result<(), String> {
        graph.must_be_directed_acyclic()?;
        if let Some(node_counts) = node_counts.as_ref() {
            node_counts
                .par_iter()
                .map(|(node_name, _)| graph.get_node_id_from_node_name(node_name).map(|_| ()))
                .collect::<Result<(), String>>()?;
        }
        if let Some(information_contents) = information_contents.as_ref() {
            if information_contents.len() == 0 {
                return Err(concat!(
                    "The provided frequencies dimensions is zero. ",
                    "The number of node frequenciess should be a strictly positive value."
                )
                .to_string());
            }

            if information_contents.len() != graph.get_number_of_nodes() as usize {
                return Err(format!(
                    concat!(
                        "The provided node frequenciess have size {}, but the expected size ",
                        "based on the provided graph {}."
                    ),
                    information_contents.len(),
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
    /// * `information_contents`: Option<&[F]> - Optional vector of node frequencies to be used WITHOUT crawling upwards the DAG.
    pub fn fit(
        &mut self,
        graph: &Graph,
        node_counts: Option<&HashMap<String, u32>>,
        information_contents: Option<&[F]>,
    ) -> Result<(), String> {
        self.validate_features(graph, node_counts, information_contents)?;
        let mut transposed_graph = graph.to_transposed();
        transposed_graph.enable(
            Some(graph.has_sources_tradeoff_enabled()),
            Some(graph.has_destinations_tradeoff_enabled()),
            Some(graph.has_cumulative_node_degrees_tradeoff_enabled()),
            Some(graph.has_reciprocal_sqrt_degrees_tradeoff_enabled()),
        )?;

        if let Some(information_contents) = information_contents {
            self.information_contents = information_contents.into();
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
            self.information_contents = node_counts
                .into_par_iter()
                .map(|node_count| -(node_count.coerce_into() / root_node_count).ln())
                .collect::<Vec<F>>();
        }
        self.root_node_ids = graph.get_root_node_ids();
        self.undirected_dag = Some(graph.to_undirected());
        Ok(())
    }

    /// Return the similarity of a given node with all others.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node for which to compute similarity against all others.
    pub fn get_similarities_from_node_id(&self, node_id: NodeT) -> Result<Vec<F>, String> {
        self.must_be_trained()?;
        if let Some(undirected_dag) = self.undirected_dag.as_ref() {
            let mut resnik_scores = vec![F::infinity(); self.get_number_of_nodes()? as usize];
            resnik_scores[node_id as usize] = self.information_contents[node_id as usize];
            let shared_resnik_scores = ThreadDataRaceAware::new(&mut resnik_scores);

            let mut predecessors = vec![NodeT::MAX; self.get_number_of_nodes()? as usize];
            predecessors[node_id as usize] = node_id;
            let shared_predecessors = NodeT::from_mut_slice(&mut predecessors);
            let mut frontier: Frontier<NodeT> = vec![node_id].into();

            while !frontier.is_empty() {
                let mut temporary_frontier: Frontier<NodeT> = Frontier::new();

                frontier.par_iter().for_each(|&src| {
                    let current_node_resnik_score =
                        unsafe { (*shared_resnik_scores.get())[src as usize] };
                    unsafe {
                        undirected_dag.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
                    }
                    .for_each(|dst| {
                        if shared_predecessors[dst as usize]
                            .compare_exchange(NodeT::MAX, src, Ordering::SeqCst, Ordering::SeqCst)
                            .is_ok()
                        {
                            // add the node to the nodes to explore
                            temporary_frontier.push(dst);
                            unsafe {
                                (*shared_resnik_scores.get())[dst as usize] =
                                    current_node_resnik_score
                                        .min(self.information_contents[dst as usize]);
                            }
                        }
                    });
                });
                frontier.clear();
                std::mem::swap(&mut frontier, &mut temporary_frontier);
            }

            return Ok(resnik_scores);
        }

        unreachable!("This is not reacheable.");
    }

    /// Return the similarity of a given node with all others.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node for which to compute similarity against all others.
    /// * `iterator`: I - Iterator over the neighbouring nodes of interest.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be worth considering.
    /// * `keep_unreacheable_nodes`: Option<bool> - Whether to keep unreacheable nodes, by default False.
    fn get_similarities_from_node_id_and_iterator<'a, I>(
        &'a self,
        node_id: NodeT,
        iterator: I,
        minimum_similarity: Option<F>,
        keep_unreacheable_nodes: Option<bool>,
    ) -> Result<impl Iterator<Item = (NodeT, F)> + '_, String>
    where
        I: Iterator<Item = NodeT> + 'a,
    {
        let keep_unreacheable_nodes = keep_unreacheable_nodes.unwrap_or(false);
        let minimum_similarity = minimum_similarity.unwrap_or(F::zero());
        let resnik_scores = self.get_similarities_from_node_id(node_id)?;
        Ok(iterator.filter_map(move |dst| {
            let score = resnik_scores[dst as usize];
            if score > minimum_similarity && (keep_unreacheable_nodes || score.is_finite()) {
                Some((dst, score))
            } else {
                None
            }
        }))
    }

    /// Return the similarity between the two provided node name prefixes.
    ///
    /// # Arguments
    /// * `first_node_prefixes`: Vec<&str> - The first node prefixes for which to compute the similarity.
    /// * `second_node_prefixes`: Vec<&str> - The second node prefixes for which to compute the similarity.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_node_ids_and_similarity_from_node_prefixes(
        &self,
        first_node_prefixes: Vec<&str>,
        second_node_prefixes: Vec<&str>,
        minimum_similarity: Option<F>,
    ) -> Result<(Vec<Vec<NodeT>>, Vec<F>), String> {
        self.must_be_trained()?;
        let task_name = format!(
            "Computing Resnik between {:?} and {:?}",
            first_node_prefixes, second_node_prefixes
        );

        let progress_bar = if self.verbose {
            let pb = ProgressBar::new(self.get_number_of_nodes()? as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(concat!(
                        "Computing Resnik ",
                        "{spinner:.green} [{elapsed_precise}] ",
                        "[{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})"
                    ))
                    .unwrap(),
            );
            pb
        } else {
            ProgressBar::hidden()
        };

        let mut progress = MarkdownFileProgress::from_project_name(task_name);
        progress.set_verbose(self.verbose);

        if let Some(graph) = self.undirected_dag.as_ref() {
            return Ok(graph
                .iter_node_ids_from_node_curie_prefixes(&first_node_prefixes)
                .progress_with_file(progress)
                .progress_with(progress_bar)
                .flat_map(|src| {
                    self.get_similarities_from_node_id_and_iterator(
                        src,
                        graph.iter_node_ids_from_node_curie_prefixes(&second_node_prefixes),
                        minimum_similarity,
                        None,
                    )
                    .unwrap()
                    .map(move |(dst, score)| (vec![src, dst], score))
                })
                .unzip());
        }

        unreachable!("Unreacheable");
    }
}
