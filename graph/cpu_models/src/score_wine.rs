use crate::wine::BasicWINE;
use crate::wine::WINEBased;
use crate::AnchorFeatureTypes;
use crate::AnchorTypes;
use crate::AnchorsInferredNodeEmbeddingModel;
use crate::AnchorsInferredNodeEmbeddingProperties;
use crate::ScoresAnchorsGenerator;

#[derive(Clone, Debug)]
pub struct ScoreWINE<'a> {
    parameters: BasicWINE,
    scores: &'a [f32],
}

impl<'a> ScoreWINE<'a> {
    pub fn new(parameters: BasicWINE, scores: &'a [f32]) -> Self {
        Self { parameters, scores }
    }
}

impl<'a> WINEBased for ScoreWINE<'a> {
    fn get_basic_wine(&self) -> &BasicWINE {
        &self.parameters
    }
}

impl<'a> ScoresAnchorsGenerator for ScoreWINE<'a> {
    fn get_scores(&self) -> &[f32] {
        self.scores.as_ref()
    }
}

impl<'a> AnchorsInferredNodeEmbeddingProperties for ScoreWINE<'a> {
    fn get_model_name(&self) -> String {
        "Score-based WINE".to_string()
    }

    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self.get_basic_inferred_node_embedding().get_embedding_size())
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicAnchorsInferredNodeEmbedding {
        self.get_basic_wine().get_basic_inferred_node_embedding()
    }
}

impl<'a>
    AnchorsInferredNodeEmbeddingModel<{ AnchorTypes::Scores }, { AnchorFeatureTypes::Walks }>
    for ScoreWINE<'a>
{
}
