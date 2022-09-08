use crate::*;

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

impl<'a> ScoresLandmarkGenerator for ScoreWINE<'a> {
    fn get_scores(&self) -> &[f32] {
        self.scores.as_ref()
    }
}

impl<'a> EmbeddingSize for ScoreWINE<'a> {
    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self
            .parameters
            .get_basic_inferred_node_embedding()
            .get_embedding_size())
    }
}

impl<'a> ALPINE<{ LandmarkType::Scores }, { LandmarkFeatureType::Windows }> for ScoreWINE<'a> {
    fn get_model_name(&self) -> String {
        "Score-based WINE".to_string()
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicALPINE {
        self.get_basic_wine().get_basic_inferred_node_embedding()
    }
}