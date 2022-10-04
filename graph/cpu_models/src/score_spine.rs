use crate::*;

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

impl<'a> ScoresLandmarkGenerator for ScoreSPINE<'a> {
    fn get_scores(&self) -> &[f32] {
        self.scores.as_ref()
    }
}

impl<'a> EmbeddingSize for ScoreSPINE<'a> {
    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self
            .get_basic_inferred_node_embedding()
            .get_embedding_size())
    }
}

impl<'a> ALPINE<{ LandmarkType::Scores }, { LandmarkFeatureType::ShortestPaths }>
    for ScoreSPINE<'a>
{
    fn get_model_name(&self) -> String {
        "Score-based SPINE".to_string()
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicALPINE {
        self.get_basic_spine().get_basic_inferred_node_embedding()
    }
}
