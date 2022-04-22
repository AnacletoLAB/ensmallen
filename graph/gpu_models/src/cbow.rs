use graph::{Graph, NodeT, WalksParameters};
use indicatif::{ProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use vec_rand::{random_f32, splitmix64};

pub struct CBOW {
    embedding_size: usize,
    window_size: usize,
    walk_parameters: WalksParameters,
    number_of_negative_samples: usize,
}

impl CBOW {
    /// Return new instance of CBOW model.
    pub fn new(
        embedding_size: Option<usize>,
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        number_of_negative_samples: Option<usize>,
    ) -> Result<Self, String> {
        // Handle the values of the default parameters.
        let embedding_size = embedding_size.unwrap_or(100);
        let window_size = window_size.unwrap_or(10);
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());
        let number_of_negative_samples = number_of_negative_samples.unwrap_or(5);

        // Validate that the provided parameters are within
        // reasonable bounds.
        if embedding_size == 0 {
            return Err(concat!("The embedding size cannot be equal to zero.").to_string());
        }
        if window_size == 0 {
            return Err(concat!("The window size cannot be equal to zero.").to_string());
        }
        if number_of_negative_samples == 0 {
            return Err(
                concat!("The number of negative samples cannot be equal to zero.").to_string(),
            );
        }

        Ok(Self {
            embedding_size,
            window_size,
            walk_parameters,
            number_of_negative_samples,
        })
    }

    pub fn fit_transform(
        &self,
        graph: &Graph,
        embedding: &mut [f32],
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        batch_size: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<(), String> {
        let epochs = epochs.unwrap_or(10);
        let batch_size = batch_size.unwrap_or(32);
        let number_of_batches_per_epoch =
            (graph.get_nodes_number() as f64 / batch_size as f64).ceil() as usize;
        let learning_rate = learning_rate.unwrap_or(0.025);
        let mut walk_parameters = self.walk_parameters.clone();
        let mut random_state = splitmix64(self.walk_parameters.get_random_state() as u64);
        let random_walk_length = walk_parameters.get_random_walk_length() as usize;
        let actual_batch_size = batch_size
            * walk_parameters.get_iterations() as usize
            * (random_walk_length - (self.window_size as usize) * 2);
        let verbose = verbose.unwrap_or(true);

        if epochs == 0 {
            return Err("The number of epochs must be strictly greater than zero.".to_string());
        }

        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
        }

        if !graph.has_nodes_sorted_by_decreasing_outbound_node_degree() {
            return Err(concat!(
                "The provided graph does not have nodes sorted by decreasing node degrees ",
                "and therefore the negative sampling used to approximate the sigmoid and ",
                "binary cross-entropy loss. You can sort this graph the desired way by ",
                "using the `graph.sort_by_decreasing_outbound_node_degree()` method. ",
                "Do note that this method does not sort in-place ",
                "but creates a new instance of the provided graph. "
            )
            .to_string());
        }

        let expected_embedding_len = self.embedding_size * graph.get_nodes_number() as usize;

        if embedding.len() != expected_embedding_len {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                embedding.len(),
                expected_embedding_len
            ));
        }

        // TODO!: Check if the requested vector sizes would even fit in GPU.
        // The check should include: embedding, hidden, batch.

        // Populate the embedding layer with random uniform values
        embedding
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, e)| *e = 2.0 * random_f32(random_state + i as u64) - 1.0);

        // TODO!: load the provided embedding in GPU.

        // Create and allocate the hidden layer
        let mut hidden = (0..expected_embedding_len)
            .into_par_iter()
            .map(|i| 2.0 * random_f32(random_state + i as u64) - 1.0)
            .collect::<Vec<_>>();

        // TODO!: load the hidden layer in GPU.

        // Create the vector we will populate with the random walks.
        let mut random_walks: Vec<NodeT> = vec![
            0;
            batch_size
                * walk_parameters.get_iterations() as usize
                * random_walk_length as usize
        ];
        // Create the vector we will be reusing multiple times
        // for the negative node IDs used to approximate a softmax
        let mut negative_node_ids: Vec<NodeT> =
            vec![0; actual_batch_size * self.number_of_negative_samples];

        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        let epochs_progress_bar = if verbose {
            let pb = ProgressBar::new(epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(
                "CBOW Epochs {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        // We start to loop over the required amount of epochs.
        for _ in (0..epochs).progress_with(epochs_progress_bar) {
            // Depending whether verbosity was requested by the user
            // we create or not a visible progress bar to show the progress
            // in the training batches.
            let batches_progress_bar = if verbose {
                let pb = ProgressBar::new(number_of_batches_per_epoch as u64);
                pb.set_style(ProgressStyle::default_bar().template(
                    "Batches {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
                ));
                pb
            } else {
                ProgressBar::hidden()
            };

            // We start to loop over the required amount of batches.
            for _ in (0..number_of_batches_per_epoch).progress_with(batches_progress_bar) {
                // We update the random state used to generate the random walks
                // and the negative samples.
                random_state = splitmix64(random_state);
                walk_parameters = walk_parameters.set_random_state(Some(random_state as usize));

                // We populate the vectors of the current training batch

                // The first part of the current training batch is constituted by the random walks
                graph.populate_random_walks_slice(
                    batch_size as NodeT,
                    &self.walk_parameters,
                    random_walks.as_mut_slice(),
                );

                // The second part by the negative node IDs
                graph
                    .par_iter_random_source_node_ids(negative_node_ids.len(), random_state)
                    .collect_into_vec(&mut negative_node_ids);

                // We move the two portions of the batch into the GPU

                // TODO! copy data in GPU
            }
        }
        Ok(())
    }
}
