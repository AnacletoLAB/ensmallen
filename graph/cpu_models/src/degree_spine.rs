use crate::*;

#[derive(Clone, Debug)]
pub struct DegreeSPINE {
    parameters: BasicSPINE,
}

impl From<BasicSPINE> for DegreeSPINE {
    fn from(parameters: BasicSPINE) -> Self {
        Self { parameters }
    }
}

impl SPINEBased for DegreeSPINE {
    fn get_basic_spine(&self) -> &BasicSPINE {
        &self.parameters
    }
}

impl EmbeddingSize for DegreeSPINE {
    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self
            .get_basic_inferred_node_embedding()
            .get_embedding_size())
    }
}

impl DegreesLandmarkGenerator for DegreeSPINE {}

impl ALPINE<{ LandmarkType::Degrees }, { LandmarkFeatureType::ShortestPaths }> for DegreeSPINE {
    fn get_model_name(&self) -> String {
        "Degree-based SPINE".to_string()
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicALPINE {
        self.get_basic_spine().get_basic_inferred_node_embedding()
    }
}
