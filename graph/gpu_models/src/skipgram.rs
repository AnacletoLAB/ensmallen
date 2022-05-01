use crate::node2vec::Node2Vec;
use crate::*;
use graph::{Graph, WalksParameters};

pub struct SkipGram {
    model: Node2Vec,
}

impl SkipGram {
    /// Return new instance of SkipGram model.
    pub fn new(
        embedding_size: Option<usize>,
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        number_of_negative_samples: Option<usize>,
    ) -> Result<Self, String> {
        Ok(Self {
            model: Node2Vec::new(
                "compute_skipgram_mini_batch",
                "SkipGram",
                embedding_size,
                walk_parameters,
                window_size,
                number_of_negative_samples,
            )?,
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.model.get_embedding_size()
    }

    pub fn fit_transform(
        &self,
        graph: &Graph,
        embedding: &mut [f32],
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        batch_size: Option<usize>,
        verbose: Option<bool>,
    ) -> Result<(), GPUError> {
        self.model
            .fit_transform(graph, embedding, epochs, learning_rate, batch_size, verbose)
    }
}
