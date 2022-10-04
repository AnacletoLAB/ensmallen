use crate::{must_not_be_zero, BasicEmbeddingModel};

#[derive(Clone, Debug)]
pub struct BasicSiameseModel {
    pub(crate) model: BasicEmbeddingModel,
    pub(crate) relu_bias: f32,
}

impl BasicSiameseModel {
    /// Return new instance of Basic Siamese Model.
    ///
    /// # Arguments
    /// * `model`: BasicEmbeddingModel - The basic embedding model.
    /// * `relu_bias`: Option<f32> - The bias to apply to the relu. By default, 1.0.
    pub fn new(model: BasicEmbeddingModel, relu_bias: Option<f32>) -> Result<Self, String> {
        Ok(Self {
            model,
            relu_bias: must_not_be_zero(relu_bias, 1.0, "ReLU bias")?,
        })
    }

    pub fn get_embedding_size(&self) -> usize {
        self.model.get_embedding_size()
    }

    pub fn get_number_of_epochs(&self) -> usize {
        self.model.get_number_of_epochs()
    }

    pub fn is_verbose(&self) -> bool {
        self.model.is_verbose()
    }

    pub fn get_dtype(&self) -> String {
        self.model.get_dtype()
    }

    pub fn get_learning_rate_decay(&self) -> f32 {
        self.model.get_learning_rate_decay()
    }

    pub fn get_learning_rate(&self) -> f32 {
        self.model.get_learning_rate()
    }

    pub fn get_random_state(&self) -> u64 {
        self.model.get_random_state()
    }
}
