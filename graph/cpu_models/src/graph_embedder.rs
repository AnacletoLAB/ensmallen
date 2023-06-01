use crate::*;
use express_measures::ThreadFloat;
use graph::{EdgeT, Graph, NodeT};
use indicatif::{ProgressBar, ProgressStyle};
use num_traits::AsPrimitive;

pub trait GraphEmbedder {
    /// Computes in the provided memory slice the graph embedding.
    ///
    /// # Arguments
    /// `graph`: &Graph - The graph to embed
    /// `embedding`: &[&mut FeatureSlice] - The memory area where to write the embedding.
    fn _fit_transform<F: ThreadFloat + 'static>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String>
    where
        f32: AsPrimitive<F>,
        NodeT: AsPrimitive<F>,
        EdgeT: AsPrimitive<F>;

    fn fit_transform<F: ThreadFloat + 'static>(
        &self,
        graph: &Graph,
        embedding: &mut [&mut [F]],
    ) -> Result<(), String>
    where
        f32: AsPrimitive<F>,
        NodeT: AsPrimitive<F>,
        EdgeT: AsPrimitive<F>,
    {
        if !graph.has_edges() {
            return Err("The provided graph does not have any edge.".to_string());
        }
        let embedding_shapes = self.get_embedding_shapes(&graph)?;
        if embedding.len() != embedding_shapes.len() {
            return Err(format!(
                concat!("We expected {} embedding, but only {} ", "were provided."),
                embedding.len(),
                embedding_shapes.len()
            ));
        }
        // Check whether the provided embeddings match with
        // the expected embedding sizes.
        for (embedding_size, (shape, expected_embedding_size)) in embedding
            .iter()
            .map(|slice| slice.len())
            .zip(embedding_shapes.iter().map(|shape| (shape, shape.size())))
        {
            if embedding_size != expected_embedding_size {
                return Err(format!(
                    concat!(
                        "The received matrix has embedding size was {} ",
                        "but the expected embedding size was {}. More ",
                        "specifically, the expected matrix shape was ({:?})."
                    ),
                    embedding_size, expected_embedding_size, shape
                ));
            }
        }

        let embedding_dimensions = embedding_shapes
            .iter()
            .map(|shape| shape[-1])
            .collect::<Vec<usize>>();

        if self.requires_random_initialization(){
            populate_vectors(embedding, &embedding_dimensions, self.get_random_state());
        }
        self._fit_transform(graph, embedding)
    }

    fn get_loading_bar(&self) -> ProgressBar {
        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        if self.is_verbose() {
            let pb = ProgressBar::new(self.get_number_of_steps() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(&format!(
                        concat!(
                            "{model_name} {{msg}} {{spinner:.green}} ",
                            "[{{elapsed_precise}}] [{{bar:40.cyan/blue}}] ",
                            "({{pos}}/{{len}}, ETA {{eta}})"
                        ),
                        model_name = self.get_model_name()
                    ))
                    .unwrap(),
            );
            pb
        } else {
            ProgressBar::hidden()
        }
    }

    /// Returns whether the model requires random initialization.
    fn requires_random_initialization(&self) -> bool;

    /// Returns whether to show the loading bar.
    fn is_verbose(&self) -> bool;

    /// Returns the name of the model.
    fn get_model_name(&self) -> String;

    /// Returns the embedding data type.
    fn get_dtype(&self) -> String;

    /// Returns the number of principal model steps.
    fn get_number_of_steps(&self) -> usize;

    /// Returns the initial random state of the model.
    fn get_random_state(&self) -> u64;

    /// Returns the shapes of the embeddings given the graph.
    /// 
    /// # Arguments
    /// * `graph`: &Graph - The graph to embed.
    fn get_embedding_shapes(&self, graph: &Graph) -> Result<Vec<MatrixShape>, String>;
}
