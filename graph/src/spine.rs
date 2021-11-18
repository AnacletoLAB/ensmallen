use super::*;
use indicatif::ProgressIterator;
use funty::IsInteger;
use log::info;
use rayon::prelude::*;
use std::convert::TryFrom;

/// # Shortest path node embedding-based algorithms.
impl Graph {

    /// Return vector of vectors of anchor node IDs, samples according to provided node centralities.
    ///
    /// # Arguments
    /// * `node_centralities`: Vec<f32> - Vector with the importance of the nodes, used to properly sample the anchors.
    /// * `embedding_size`: usize - The number of features to sample for.
    /// * `quantile`: f32 - Percentage of the nodes to sample.
    /// * `verbose`: bool - Whether to show the loading bar.
    ///
    /// # Raises
    /// * If the provided node centrality distribution is not amongst the supported ones.
    fn get_anchor_node_ids_from_node_centralities(
        &self,
        node_centralities: &[f32],
        embedding_size: usize,
        quantile: f32,
        verbose: bool,
    ) -> Result<Vec<Vec<NodeT>>> {
        info!("Computing sum of node features.");
        let total_node_features: f32 = node_centralities.par_iter().sum();
        
        // Compute the threshold
        let threshold = total_node_features * (1.0 - quantile);
        let mut current_total = 0.0;

        info!("Sorting centralities.");
        let mut node_ids: Vec<NodeT> = self.get_node_ids();
        node_ids.par_sort_unstable_by(|&a, &b| {
            node_centralities[b as usize]
                .partial_cmp(&node_centralities[a as usize])
                .unwrap()
        });
        let embedding_size = embedding_size.min(node_ids.len());
        info!("Starting to compute anchors.");
        // Allocate the buckets centralities scores
        let mut bucket_centralities = vec![0.0; embedding_size];
        // Allocate the node scores
        let mut buckets: Vec<Vec<NodeT>> = (0..embedding_size).map(|_| Vec::new()).collect();
        // Start to properly iterate
        let pb = get_loading_bar(
            verbose,
            "Computing anchors",
            embedding_size
        );
        node_ids.into_iter().progress_with(pb).for_each(|node_id|{
            if current_total < threshold {
                let (argmin, _) = bucket_centralities.par_iter().argmin().unwrap();
                bucket_centralities[argmin] += node_centralities[node_id as usize];
                current_total += node_centralities[node_id as usize];
                buckets[argmin].push(node_id);    
            }
        });

        Ok(buckets)

    }

    #[manual_binding]
    /// Return node embedding vector obtained from shortest-paths.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - The number of features to generate. By default 100, or the number of nodes in the graph if it is lower.
    /// * `number_of_central_nodes_to_sample`: Option<usize> - The number of nodes of high degree to initially sample. By default 10, or the number of nodes in the graph if it is lower.
    /// * `quantile`: Option<f32> - The top quantile of nodes to sample after weighting. By default, the top 20%.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    pub fn get_spine<'a, T: 'a + TryFrom<u32> + Into<u32> + Send + Sync + IsInteger + TryFrom<usize>>(
        &'a self,
        embedding_size: Option<usize>,
        number_of_central_nodes_to_sample: Option<usize>,
        quantile: Option<f32>,
        verbose: Option<bool>,
    ) -> Result<(NodeT, impl Iterator<Item = impl IndexedParallelIterator<Item = T> + 'a> + 'a)> {
        let embedding_size = embedding_size.unwrap_or(100.min(self.get_nodes_number() as usize));

        if embedding_size < 1{
            return Err(format!(
                concat!(
                    "The embedding size cannot be less than one. ",
                    "The value you provided was {}."
                ),
                embedding_size
            ));
        }

        let number_of_central_nodes_to_sample = number_of_central_nodes_to_sample.unwrap_or(10.min(self.get_nodes_number() as usize));
        
        if number_of_central_nodes_to_sample < 1{
            return Err(format!(
                concat!(
                    "The number of central nodes to sample cannot be less than one. ",
                    "The value you provided was {}."
                ),
                number_of_central_nodes_to_sample
            ));
        }

        let quantile = quantile.unwrap_or(0.2);

        if quantile <= 0.0 || quantile >= 1.0{
            return Err(format!(
                concat!(
                    "The provided quantile must be between 0 and 1, while ",
                    "the quantile you provided was {}."
                ),
                quantile
            ));
        }

        let verbose = verbose.unwrap_or(true);

        // Compute the top k nodes
        info!("Computing top {} node ids.", number_of_central_nodes_to_sample);
        let central_node_ids = self.get_top_k_central_node_ids(number_of_central_nodes_to_sample as NodeT)?;
        
        // Compute the nodes degree centralities
        info!("Computing node degree centralities.");
        let mut node_centralities = self.get_degree_centrality()?;

        // Iterate over the most high degree nodes
        // and update the node centralities according
        // to their distance.
        info!("Starting to approximated harmonic centralities.");
        let pb = get_loading_bar(
            verbose, 
            "Compute approximated harmonic centralities",
            number_of_central_nodes_to_sample
        );

        central_node_ids.into_iter().progress_with(pb).for_each(|node_id| unsafe{
            self.get_unchecked_generic_breadth_first_search_distances_parallel_from_node_ids::<T>(
                vec![node_id], 
                None
            ).0.into_par_iter()
            .zip(node_centralities.par_iter_mut())
            .for_each(|(distance, node_centrality)|{
                    let distance: u32 = distance.into();
                    if distance > 0 {
                        *node_centrality *= distance as f32;    
                }
            });
        });

        // Compute the anchor node IDs.
        let anchor_node_ids = self.get_anchor_node_ids_from_node_centralities(
            &node_centralities,
            embedding_size,
            quantile,
            verbose,
        )?;

        info!("Starting to compute node features.");
        let pb = get_loading_bar(
            verbose, 
            "Compute approximated harmonic centralities",
            number_of_central_nodes_to_sample
        );
        
        Ok((
            anchor_node_ids.len() as NodeT,
            anchor_node_ids
                .into_iter()
                .progress_with(pb)
                .map(move |anchor_node_ids| unsafe {
                        self.get_unchecked_generic_breadth_first_search_distances_parallel_from_node_ids::<T>(
                            anchor_node_ids,
                            None,
                        ).0.into_par_iter()
                })
        ))
    }
}
