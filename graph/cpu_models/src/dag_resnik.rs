use graph::{Graph, NodeT, ThreadDataRaceAware};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use num_traits::{Coerced, Float, IntoAtomic};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub struct DAGResnik<F: Float> {
    /// The DAG to be used to search successors.
    dag: Option<Graph>,
    /// The transposed DAG to be used to search preprocessors.
    transposed_dag: Option<Graph>,
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
            dag: None,
            transposed_dag: None,
            root_node_ids: Vec::new(),
            information_contents: Vec::new(),
            verbose: verbose.unwrap_or(true),
        }
    }

    fn must_be_trained(&self) -> Result<(&Graph, &Graph), String> {
        if let (Some(dag), Some(transposed_dag)) = (self.dag.as_ref(), self.transposed_dag.as_ref())
        {
            Ok((dag, transposed_dag))
        } else {
            Err(concat!(
                "This model has not been trained yet. ",
                "You should call the `.fit` method first."
            )
            .to_string())
        }
    }

    /// Returns the node frequencies of the model.
    pub fn get_number_of_nodes(&self) -> Result<NodeT, String> {
        self.must_be_trained()
            .map(|(dag, _)| dag.get_number_of_nodes())
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
    /// * `dag`: &Graph - The dag whose edges are to be learned.
    /// * `node_counts`: Option<HashMap<String, u32>> - Hashmap of node counts. These counts should represent how many times a given node appears in a set.
    /// * `information_contents`: Option<&[F]> - Optional vector of node frequencies to be used WITHOUT crawling upwards the DAG.
    pub fn fit(
        &mut self,
        dag: &Graph,
        node_counts: Option<&HashMap<String, u32>>,
        information_contents: Option<&[F]>,
    ) -> Result<(), String> {
        self.validate_features(&dag, node_counts, information_contents)?;
        let mut transposed_dag = dag.to_transposed();
        transposed_dag.enable(
            Some(dag.has_sources_tradeoff_enabled()),
            Some(dag.has_destinations_tradeoff_enabled()),
            Some(dag.has_cumulative_node_degrees_tradeoff_enabled()),
            Some(dag.has_reciprocal_sqrt_degrees_tradeoff_enabled()),
        )?;

        if let Some(information_contents) = information_contents {
            self.information_contents = information_contents.into();
        } else {
            let mut node_counts = if let Some(node_counts) = node_counts {
                dag.par_iter_node_names()
                    .map(|node_name| *node_counts.get(&node_name).unwrap_or(&0))
                    .collect::<Vec<u32>>()
            } else {
                vec![1; dag.get_number_of_nodes() as usize]
            };
            let visited_by_all_child = unsafe {
                std::mem::transmute::<Vec<bool>, Vec<AtomicBool>>(vec![
                    false;
                    dag.get_number_of_nodes()
                        as usize
                ])
            };
            let mut frontier = dag
                .par_iter_trap_node_ids()
                .flat_map_iter(|leaf_node_id| unsafe {
                    visited_by_all_child[leaf_node_id as usize].store(true, Ordering::SeqCst);
                    transposed_dag
                        .iter_unchecked_neighbour_node_ids_from_source_node_id(leaf_node_id)
                })
                .collect::<Vec<NodeT>>();
            while !frontier.is_empty() {
                frontier = frontier
                    .into_iter()
                    .flat_map(|node_id| unsafe {
                        // If any of the children nodes of this node
                        // were not visited
                        if dag
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
                        node_counts[node_id as usize] = dag
                            .iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)
                            .map(|child_node_id| node_counts[child_node_id as usize])
                            .sum::<u32>();
                        // And we return its parents as the next nodes to be visited.
                        transposed_dag
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
        self.root_node_ids = dag.get_root_node_ids();
        self.transposed_dag = Some(dag.to_transposed());
        self.dag = Some(dag.clone());
        Ok(())
    }

    /// Return the similarity of a given node with all others.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - The node for which to compute similarity against all others.
    pub fn get_similarities_from_node_id(&self, node_id: NodeT) -> Result<Vec<F>, String> {
        self.must_be_trained().map(|(dag, transposed_dag)| {
            let mut resnik_scores =
                vec![F::infinity(); self.get_number_of_nodes().unwrap() as usize];
            resnik_scores[node_id as usize] = self.information_contents[node_id as usize];
            let shared_resnik_scores = ThreadDataRaceAware::new(&mut resnik_scores);

            let mut predecessors = vec![NodeT::MAX; self.get_number_of_nodes().unwrap() as usize];
            predecessors[node_id as usize] = node_id;
            let shared_predecessors = NodeT::from_mut_slice(&mut predecessors);
            let mut frontier: Vec<NodeT> = vec![node_id];
            let mut downward_frontier: Vec<NodeT> = Vec::new();

            while !frontier.is_empty() {
                let mut temporary_frontier: Vec<NodeT> = Vec::new();
                let mut temporary_downward_frontier: Vec<NodeT> = Vec::new();

                frontier.iter().for_each(|&src| {
                    let current_node_resnik_score =
                        unsafe { (*shared_resnik_scores.get())[src as usize] };
                    // First we handle the explorations upward, towards to head of the dag.
                    unsafe {
                        transposed_dag.iter_unchecked_neighbour_node_ids_from_source_node_id(src)
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

                    // Then we handle the downward exploration.
                    unsafe { dag.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                        .for_each(|dst| {
                            if shared_predecessors[dst as usize]
                                .compare_exchange(
                                    NodeT::MAX,
                                    src,
                                    Ordering::SeqCst,
                                    Ordering::SeqCst,
                                )
                                .is_ok()
                            {
                                // add the node to the nodes to explore
                                temporary_downward_frontier.push(dst);
                                unsafe {
                                    (*shared_resnik_scores.get())[dst as usize] =
                                        current_node_resnik_score
                                            .min(self.information_contents[dst as usize]);
                                }
                            }
                        });
                });

                downward_frontier.iter().for_each(|&src| {
                    let current_node_resnik_score =
                        unsafe { (*shared_resnik_scores.get())[src as usize] };
                    // Then we handle the downward exploration.
                    unsafe { dag.iter_unchecked_neighbour_node_ids_from_source_node_id(src) }
                        .for_each(|dst| {
                            if shared_predecessors[dst as usize]
                                .compare_exchange(
                                    NodeT::MAX,
                                    src,
                                    Ordering::SeqCst,
                                    Ordering::SeqCst,
                                )
                                .is_ok()
                            {
                                // add the node to the nodes to explore
                                temporary_downward_frontier.push(dst);
                                unsafe {
                                    (*shared_resnik_scores.get())[dst as usize] =
                                        current_node_resnik_score
                                            .min(self.information_contents[dst as usize]);
                                }
                            }
                        });
                });
                frontier.clear();
                downward_frontier.clear();
                std::mem::swap(&mut frontier, &mut temporary_frontier);
                std::mem::swap(&mut downward_frontier, &mut temporary_downward_frontier);
            }

            resnik_scores
        })
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
    ) -> Result<impl ParallelIterator<Item = (NodeT, F)> + '_, String>
    where
        I: ParallelIterator<Item = NodeT> + 'a,
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

    /// Return the similarity between the two parallel iterators of node Ids.
    ///
    /// # Arguments
    /// * `first_iterator`: fn(&Graph, &A1) -> Result<I1, String> - Second generator of iterators.
    /// * `first_attribute`: &A1 - Parameter to be forwarded to the first iterator generation method.
    /// * `second_iterator`: fn(&Graph, &A2) -> Result<I2, String> - Second generator of iterators.
    /// * `second_attribute`: &A1 - Parameter to be forwarded to the first iterator generation method.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    fn get_node_ids_and_similarity_from_iterators<'a, 'b, I1, I2, A1, A2>(
        &'a self,
        first_iterator: fn(&'b Graph, &'b A1) -> Result<I1, String>,
        first_attribute: &'b A1,
        second_iterator: fn(&'a Graph, &'a A2) -> I2,
        second_attribute: &'a A2,
        minimum_similarity: Option<F>,
    ) -> Result<(Vec<Vec<NodeT>>, Vec<F>), String>
    where
        I1: ParallelIterator<Item = NodeT> + 'a,
        I2: ParallelIterator<Item = NodeT>,
        A2: Sync,
        'a: 'b,
    {
        self.must_be_trained().and_then(|(dag, _)| {
            let progress_bar = if self.verbose {
                let pb = ProgressBar::new(first_iterator(&dag, first_attribute)?.count() as u64);
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

            Ok(first_iterator(&dag, first_attribute)?
                .progress_with(progress_bar)
                .flat_map(|src| {
                    self.get_similarities_from_node_id_and_iterator(
                        src,
                        second_iterator(&dag, second_attribute),
                        minimum_similarity,
                        None,
                    )
                    .unwrap()
                    .map(move |(dst, score)| (vec![src, dst], score))
                })
                .unzip())
        })
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
        self.get_node_ids_and_similarity_from_iterators(
            |graph, prefixes| Ok(graph.par_iter_node_ids_from_node_curie_prefixes(&prefixes)),
            &first_node_prefixes,
            |graph, prefixes| graph.par_iter_node_ids_from_node_curie_prefixes(&prefixes),
            &second_node_prefixes,
            minimum_similarity,
        )
    }

    /// Return the similarity between the two provided node type names.
    ///
    /// # Arguments
    /// * `first_node_type_names`: &[Option<&str>] - The first node type names for which to compute the similarity.
    /// * `second_node_type_names`: &[Option<&str>] - The second node type names for which to compute the similarity.
    /// * `minimum_similarity`: Option<F> - Minimum similarity to be kept. Values below this amount are filtered.
    pub fn get_node_ids_and_similarity_from_node_type_names(
        &self,
        first_node_type_names: &[Option<&str>],
        second_node_type_names: &[Option<&str>],
        minimum_similarity: Option<F>,
    ) -> Result<(Vec<Vec<NodeT>>, Vec<F>), String> {
        self.get_node_ids_and_similarity_from_iterators(
            |graph, node_type_names| graph.par_iter_node_ids_from_node_type_names(&node_type_names),
            &first_node_type_names,
            |graph, node_type_names| {
                graph
                    .par_iter_node_ids_from_node_type_names(&node_type_names)
                    .unwrap()
            },
            &second_node_type_names,
            minimum_similarity,
        )
    }
}
