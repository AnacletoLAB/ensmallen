use crate::*;

#[derive(Clone, Debug)]
pub struct DegreeWINE {
    parameters: BasicWINE,
}

impl From<BasicWINE> for DegreeWINE {
    fn from(parameters: BasicWINE) -> Self {
        Self { parameters }
    }
}

impl WINEBased for DegreeWINE {
    fn get_basic_wine(&self) -> &BasicWINE {
        &self.parameters
    }
}

impl EmbeddingSize for DegreeWINE {
    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self
            .parameters
            .get_basic_inferred_node_embedding()
            .get_embedding_size())
    }
}

impl DegreesLandmarkGenerator for DegreeWINE {}

impl ALPINE<{ LandmarkType::Degrees }, { LandmarkFeatureType::Windows }> for DegreeWINE {
    fn get_model_name(&self) -> String {
        "Degree-based WINE".to_string()
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicALPINE {
        self.get_basic_wine().get_basic_inferred_node_embedding()
    }
}