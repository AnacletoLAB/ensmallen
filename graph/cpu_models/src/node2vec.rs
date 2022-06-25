use crate::*;
use graph::WalksParameters;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node2VecModels {
    CBOW,
    SkipGram,
}

impl std::fmt::Display for Node2VecModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct Node2Vec<W>
where
    W: WalkTransformer,
{
    pub(crate) embedding_size: usize,
    pub(crate) walk_parameters: WalksParameters,
    pub(crate) window_size: usize,
    pub(crate) clipping_value: f32,
    pub(crate) number_of_negative_samples: usize,
    pub(crate) epochs: usize,
    pub(crate) learning_rate: f32,
    pub(crate) learning_rate_decay: f32,
    pub(crate) stochastic_downsample_by_degree: bool,
    pub(crate) normalize_learning_rate_by_degree: bool,
    pub(crate) use_zipfian_sampling: bool,
    pub(crate) walk_transformer: W,
    pub(crate) model_type: Node2VecModels,
    pub(crate) verbose: bool,
}

impl<W> Node2Vec<W>
where
    W: WalkTransformer,
{
    /// Return new instance of Node2Vec model.
    ///
    /// # Arguments
    /// `model_type`: Node2VecModels - The model to be used.
    /// `walk_transformer`: W - Transformation to apply to the random walks.
    /// `embedding_size`: Option<usize> - Size of the embedding.
    /// `walk_parameters`: Option<WalksParameters> - Parameters to be used within the walks.
    /// `window_size`: Option<usize> - Window size defining the contexts.
    /// `clipping_value`: Option<f32> - Value at which we clip the dot product, mostly for numerical stability issues. By default, `6.0`, where the loss is already close to zero.
    /// `number_of_negative_samples`: Option<usize> - Number of negative samples to extract for each context.
    /// `epochs`: Option<usize> - The number of epochs to run the model for, by default 10.
    /// `learning_rate`: Option<f32> - The learning rate to update the gradient, by default 0.01.
    /// `learning_rate_decay`: Option<f32> - Factor to reduce the learning rate for at each epoch. By default 0.9.
    /// `stochastic_downsample_by_degree`: Option<bool> - Randomly skip samples with probability proportional to the degree of the central node. By default false.
    /// `normalize_learning_rate_by_degree`: Option<bool> - Divide the learning rate by the degree of the central node. By default false.
    /// `use_zipfian_sampling`: Option<bool> - Sample negatives proportionally to their degree. By default true.
    /// `verbose`: Option<bool> - Whether to show the loading bar, by default true.
    pub fn new(
        model_type: Node2VecModels,
        walk_transformer: W,
        embedding_size: Option<usize>,
        walk_parameters: Option<WalksParameters>,
        window_size: Option<usize>,
        clipping_value: Option<f32>,
        number_of_negative_samples: Option<usize>,
        epochs: Option<usize>,
        learning_rate: Option<f32>,
        learning_rate_decay: Option<f32>,
        stochastic_downsample_by_degree: Option<bool>,
        normalize_learning_rate_by_degree: Option<bool>,
        use_zipfian_sampling: Option<bool>,
        verbose: Option<bool>,
    ) -> Result<Self, String> {
        let embedding_size = must_not_be_zero(embedding_size, 100, "embedding size")?;
        let window_size = must_not_be_zero(window_size, 10, "window size")?;
        let clipping_value = must_not_be_zero(clipping_value, 6.0, "clipping value")?;
        let number_of_negative_samples =
            must_not_be_zero(number_of_negative_samples, 5, "number of negative samples")?;
        let epochs = must_not_be_zero(epochs, 10, "epochs")?;
        let learning_rate = must_not_be_zero(learning_rate, 0.01, "learning rate")?;
        let learning_rate_decay =
            must_not_be_zero(learning_rate_decay, 0.9, "learning rate decay")?;
        let walk_parameters = walk_parameters.unwrap_or_else(|| WalksParameters::default());
        let stochastic_downsample_by_degree = stochastic_downsample_by_degree.unwrap_or(false);
        let normalize_learning_rate_by_degree = normalize_learning_rate_by_degree.unwrap_or(false);
        let use_zipfian_sampling = use_zipfian_sampling.unwrap_or(true);
        let verbose = verbose.unwrap_or(true);

        Ok(Self {
            model_type,
            embedding_size,
            window_size,
            walk_parameters,
            clipping_value,
            epochs,
            learning_rate,
            learning_rate_decay,
            number_of_negative_samples,
            stochastic_downsample_by_degree,
            normalize_learning_rate_by_degree,
            use_zipfian_sampling,
            walk_transformer,
            verbose,
        })
    }

    /// Returns the used embedding size.
    pub fn get_embedding_size(&self) -> usize {
        self.embedding_size
    }

    pub(crate) fn get_progress_bar(&self) -> ProgressBar {
        // Depending whether verbosity was requested by the user
        // we create or not a visible progress bar to show the progress
        // in the training epochs.
        if self.verbose {
            let pb = ProgressBar::new(self.epochs as u64);
            pb.set_style(ProgressStyle::default_bar().template(&format!(
                concat!(
                    "{}{{msg}} {{spinner:.green}} [{{elapsed_precise}}] ",
                    "[{{bar:40.cyan/blue}}] ({{pos}}/{{len}}, ETA {{eta}})"
                ),
                self.get_model_name()
            )));
            pb
        } else {
            ProgressBar::hidden()
        }
    }
}

impl<W> GraphEmbedder<f32> for Node2Vec<W>
where
    W: WalkTransformer,
{
    fn get_embedding_sizes(&self, graph: &graph::Graph) -> Vec<(usize, usize)> {
        vec![(graph.get_nodes_number() as usize, self.embedding_size)]
    }

    fn get_model_name(&self) -> String {
        self.model_type.to_string()
    }

    fn fit_transform(
        &self,
        graph: &graph::Graph,
        embedding: &mut [&mut [f32]],
    ) -> Result<(), String> {
        if !graph.has_nodes() {
            return Err("The provided graph does not have any node.".to_string());
        }
        let nodes_number = graph.get_nodes_number();
        let expected_embedding_len = self.embedding_size * nodes_number as usize;

        if embedding[0].len() != expected_embedding_len {
            return Err(format!(
                "The given memory allocation for the embeddings is {} long but we expect {}.",
                embedding[0].len(),
                expected_embedding_len
            ));
        }
        match self.model_type {
            Node2VecModels::CBOW => self.fit_transform_cbow(graph, embedding),
            Node2VecModels::SkipGram => self.fit_transform_skipgram(graph, embedding),
        }
    }
}
