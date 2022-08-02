use crate::spine::BasicSPINE;
use crate::spine::SPINEBased;
use crate::AnchorFeatureTypes;
use crate::AnchorTypes;
use crate::AnchorsInferredNodeEmbeddingModel;
use crate::AnchorsInferredNodeEmbeddingProperties;
use crate::ScoresAnchorsGenerator;

#[derive(Clone, Debug)]
pub struct ScoreSPINE<'a> {
    parameters: BasicSPINE,
    scores: &'a [f32],
}

impl<'a> ScoreSPINE<'a> {
    pub fn new(parameters: BasicSPINE, scores: &'a [f32]) -> Self {
        Self { parameters, scores }
    }
}

impl<'a> SPINEBased for ScoreSPINE<'a> {
    fn get_basic_spine(&self) -> &BasicSPINE {
        &self.parameters
    }
}

impl<'a> ScoresAnchorsGenerator for ScoreSPINE<'a> {
    fn get_scores(&self) -> &[f32] {
        self.scores.as_ref()
    }
}

impl<'a> AnchorsInferredNodeEmbeddingProperties for ScoreSPINE<'a> {
    fn get_model_name(&self) -> String {
        "Score-based SPINE".to_string()
    }

    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self.get_basic_inferred_node_embedding().get_embedding_size())
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicAnchorsInferredNodeEmbedding {
        self.get_basic_spine().get_basic_inferred_node_embedding()
    }
}

impl<'a>
    AnchorsInferredNodeEmbeddingModel<{ AnchorTypes::Scores }, { AnchorFeatureTypes::ShortestPaths }>
    for ScoreSPINE<'a>
{
}
