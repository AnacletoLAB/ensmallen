use crate::wine::BasicWINE;
use crate::wine::WINEBased;
use crate::AnchorTypes;
use crate::AnchorFeatureTypes;
use crate::AnchorsInferredNodeEmbeddingModel;
use crate::AnchorsInferredNodeEmbeddingProperties;
use crate::NodeTypesAnchorsGenerator;

#[derive(Clone, Debug)]
pub struct NodeLabelWINE {
    parameters: BasicWINE,
}

impl From<BasicWINE> for NodeLabelWINE {
    fn from(parameters: BasicWINE) -> Self {
        Self { parameters }
    }
}

impl WINEBased for NodeLabelWINE {
    fn get_basic_wine(&self) -> &BasicWINE {
        &self.parameters
    }
}

impl NodeTypesAnchorsGenerator for NodeLabelWINE {}
impl AnchorsInferredNodeEmbeddingProperties for NodeLabelWINE {
    fn get_model_name(&self) -> String {
        "Node-types-based WINE".to_string()
    }

    fn get_embedding_size(&self, graph: &graph::Graph) -> Result<usize, String> {
        Ok(graph.get_number_of_node_types()? as usize)
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicAnchorsInferredNodeEmbedding {
        self.get_basic_wine().get_basic_inferred_node_embedding()
    }
}

impl AnchorsInferredNodeEmbeddingModel<{ AnchorTypes::NodeTypes }, {AnchorFeatureTypes::Walks}> for NodeLabelWINE {}
