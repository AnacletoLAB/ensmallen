use crate::AnchorFeatureTypes;
use crate::wine::BasicWINE;
use crate::wine::WINEBased;
use crate::AnchorTypes;
use crate::AnchorsInferredNodeEmbeddingModel;
use crate::AnchorsInferredNodeEmbeddingProperties;
use crate::DegreesAnchorsGenerator;

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

impl DegreesAnchorsGenerator for DegreeWINE {}
impl AnchorsInferredNodeEmbeddingProperties for DegreeWINE {
    fn get_model_name(&self) -> String {
        "Degree-based WINE".to_string()
    }

    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self.get_basic_inferred_node_embedding().get_embedding_size())
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicAnchorsInferredNodeEmbedding {
        self.get_basic_wine().get_basic_inferred_node_embedding()
    }
}

impl AnchorsInferredNodeEmbeddingModel<{ AnchorTypes::Degrees }, {AnchorFeatureTypes::Walks}> for DegreeWINE {}
