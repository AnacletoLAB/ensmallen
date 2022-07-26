use crate::AnchorFeatureTypes;
use crate::spine::BasicSPINE;
use crate::spine::SPINEBased;
use crate::AnchorTypes;
use crate::AnchorsInferredNodeEmbeddingModel;
use crate::AnchorsInferredNodeEmbeddingProperties;
use crate::DegreesAnchorsGenerator;

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

impl DegreesAnchorsGenerator for DegreeSPINE {}
impl AnchorsInferredNodeEmbeddingProperties for DegreeSPINE {
    fn get_model_name(&self) -> String {
        "Degree-based SPINE".to_string()
    }

    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self.get_basic_inferred_node_embedding().get_embedding_size())
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicAnchorsInferredNodeEmbedding {
        self.get_basic_spine().get_basic_inferred_node_embedding()
    }
}

impl AnchorsInferredNodeEmbeddingModel<{ AnchorTypes::Degrees }, {AnchorFeatureTypes::ShortestPaths}> for DegreeSPINE {}
