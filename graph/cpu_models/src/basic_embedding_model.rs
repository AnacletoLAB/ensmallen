use crate::must_not_be_zero;

#[derive(Clone, Debug)]
pub struct BasicEmbeddingModel {
    pub(crate) embedding_size: usize,
    pub(crate) random_state: u64,
    pub(crate) epochs: usize,
    pub(crate) learning_rate: f32,
    pub(crate) learning_rate_decay: f32,
    pub(crate) verbose: bool,
}

impl BasicEmbeddingModel {
    /// Return new instance of Basic Embedding Model.
    ///
    /// # Arguments
    /// * `embedding_size`: Option<usize> - Size of the embedding.
    /// * `epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// * `learning_rate`: Option<f32> - The learning rate to update the gradient, by default 0.01.
    /// * `learning_rate_decay`: Option<f32> - Factor to reduce the learning rate for at each epoch. By default 0.9.
    /// * `random_state`: Option<u64> - The random state to use to reproduce the training.
    /// * `verbose`: Option<bool> - Whether to show loading bar.
    pub fn new(
        embedding_size: Option<usize>,
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        learning_rate_decay: Option<f32>,
        random_state: Option<u64>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        Ok(Self {
            embedding_size: must_not_be_zero(embedding_size, 100, "embedding size")?,
            epochs: must_not_be_zero(epochs, 10, "epochs")?,
            learning_rate: must_not_be_zero(learning_rate, 0.001, "learning rate")?,
            learning_rate_decay: must_not_be_zero(
                learning_rate_decay,
                0.99,
                "learning rate decay",
            )?,
            random_state: random_state.unwrap_or(42),
            verbose: verbose.unwrap_or(true),
        })
    }

    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    pub fn get_number_of_epochs(&self) -> usize {
        self.epochs
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_learning_rate_decay(&self) -> f32 {
        self.learning_rate_decay
    }

    pub fn get_learning_rate(&self) -> f32 {
        self.learning_rate
    }

    pub fn get_random_state(&self) -> u64 {
        self.random_state
    }
}
