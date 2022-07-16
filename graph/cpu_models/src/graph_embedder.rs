use crate::populate_vectors;
use graph::Graph;
use indicatif::{ProgressBar, ProgressStyle};

pub trait GraphEmbedder {
    /// Computes in the provided memory slice the graph embedding.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &[&mut [f32]] - The memory area where to write the embedding.
    fn _fit_transform(&self, graph: &Graph, embedding: &mut [&mut [f32]]) -> Result<(), String>;

    fn fit_transform(&self, graph: &Graph, embedding: &mut [&mut [f32]]) -> Result<(), String> {
        if !graph.has_edges() {
            return Err("The provided graph does not have any edge.".to_string());
        }
        let embedding_shapes = self.get_embedding_shapes(&graph)?;
        if embedding.len() != embedding_shapes.len(){
            return Err(format!(
                concat!(
                    "We expected {} embedding, but only {} ",
                    "were provided."
                ),
                embedding.len(), embedding_shapes.len()
            ));
        }
        // Check whether the provided embeddings match with
        // the expected embedding sizes.
        for (embedding_size, (x, y, expected_embedding_size)) in embedding
            .iter()
            .map(|slice| slice.len())
            .zip(embedding_shapes.iter().map(|(x, y)| (*x, *y, *x * *y)))
        {
            if embedding_size != expected_embedding_size {
                return Err(format!(
                    concat!(
                        "The received matrix has embedding size was {} ",
                        "but the expected embedding size was {}. More ",
                        "specifically, the expected matrix shape was ({}, {})."
                    ),
                    embedding_size, expected_embedding_size, x, y
                ));
            }
        }

        populate_vectors(
            embedding,
            self.get_random_state(),
            embedding_shapes
                .into_iter()
                .map(|(_, embedding_size)| (embedding_size as f32).sqrt())
                .collect::<Vec<f32>>()
                .as_slice(),
        );
        self._fit_transform(graph, embedding)
    }

    fn get_loading_bar(&self) -> ProgressBar {
        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        if self.is_verbose() {
            let pb = ProgressBar::new(self.get_number_of_epochs() as u64);
            pb.set_style(ProgressStyle::default_bar().template(&format!(
                concat!(
                    "{model_name} {{msg}} {{spinner:.green}} ",
                    "[{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ",
                    "({{pos}}/{{len}}, ETA {{eta}})"
                ),
                model_name = self.get_model_name()
            )));
            pb
        } else {
            ProgressBar::hidden()
        }
    }

    /// Returns whether to show the loading bar.
    fn is_verbose(&self) -> bool;

    /// Returns the name of the model.
    fn get_model_name(&self) -> String;

    /// Returns the number of epochs.
    fn get_number_of_epochs(&self) -> usize;

    /// Returns the initial random state of the model.
    fn get_random_state(&self) -> u64;

    /// Returns the shapes of the embeddings given the graph.
    fn get_embedding_shapes(&self, graph: &Graph) -> Result<Vec<(usize, usize)>, String>;
}
