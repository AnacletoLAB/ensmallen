use super::*;
use funty::IsInteger;
use indicatif::ProgressIterator;
use log::info;
use rayon::prelude::*;
use std::convert::TryFrom;

/// # Shortest path node embedding-based algorithms.
impl Graph {
    /// Return vector of vectors of anchor node IDs.
    ///
    /// # Arguments
    /// * `embedding_size`: usize - The number of features to sample for.
    ///
    fn get_anchor_node_ids(&self, embedding_size: usize) -> Result<Vec<Vec<NodeT>>> {
        info!("Computing sum of node features.");
        let number_of_edge_per_bucket: EdgeT =
            (self.get_directed_edges_number() as f32 / 2 as f32 / embedding_size as f32).floor()
                as EdgeT;

        info!("Sorting centralities.");
        let mut node_ids: Vec<NodeT> = self.get_node_ids();
        node_ids.par_sort_unstable_by(|&a, &b| unsafe {
            self.get_unchecked_node_degree_from_node_id(b)
                .partial_cmp(&self.get_unchecked_node_degree_from_node_id(a))
                .unwrap()
        });
        info!("Starting to compute anchors.");
        // Allocate the node scores
        let mut current_bucket_size = 0;
        let mut current_bucket_index = 0;
        let mut buckets: Vec<Vec<NodeT>> = (0..embedding_size).map(|_| Vec::new()).collect();
        node_ids.into_iter().for_each(|node_id| unsafe {
            if current_bucket_size > number_of_edge_per_bucket {
                current_bucket_size = 0;
                current_bucket_index += 1;
            }
            if current_bucket_index == embedding_size {
                return;
            }
            current_bucket_size += self.get_unchecked_node_degree_from_node_id(node_id) as EdgeT;
            buckets[current_bucket_index].push(node_id);
        });

        Ok(buckets)
    }

    #[manual_binding]
    /// Return node embedding vector obtained from shortest-paths.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - The number of features to generate. By default 100, or the number of nodes in the graph if it is lower.
    /// * `verbose`: Option<bool> - Whether to show the loading bar. By default true.
    ///
    pub fn get_spine<
        'a,
        T: 'a + TryFrom<u32> + Into<u32> + Send + Sync + IsInteger + TryFrom<usize>,
    >(
        &'a self,
        embedding_size: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<(
        NodeT,
        impl Iterator<Item = impl IndexedParallelIterator<Item = T> + 'a> + 'a,
    )> {
        let embedding_size = embedding_size.unwrap_or(100.min(self.get_nodes_number() as usize));

        if embedding_size < 1 {
            return Err(format!(
                concat!(
                    "The embedding size cannot be less than one. ",
                    "The value you provided was {}."
                ),
                embedding_size
            ));
        }

        let verbose = verbose.unwrap_or(true);

        // Compute the anchor node IDs.
        let anchor_node_ids = self.get_anchor_node_ids(embedding_size)?;

        info!("Starting to compute node features.");
        let pb = get_loading_bar(verbose, "Computing node features", anchor_node_ids.len());

        Ok((
            anchor_node_ids.len() as NodeT,
            anchor_node_ids
                .into_iter()
                .progress_with(pb)
                .map(move |anchor_node_ids| unsafe {
                    let (distances, eccentricity, _) = self.get_unchecked_generic_breadth_first_search_distances_parallel_from_node_ids::<T>(
                        anchor_node_ids,
                        None,
                    );
                    distances.into_par_iter().map(move |distance| if distance > eccentricity {
                        eccentricity
                    } else {
                        distance
                    })
                })
        ))
    }
}