use crate::spine::BasicSPINE;
use crate::spine::SPINEBased;
use crate::AnchorTypes;
use crate::AnchorFeatureTypes;
use crate::AnchorsInferredNodeEmbeddingModel;
use crate::AnchorsInferredNodeEmbeddingProperties;
use crate::NodeTypesAnchorsGenerator;

#[derive(Clone, Debug)]
pub struct NodeLabelSPINE {
    parameters: BasicSPINE,
}

impl From<BasicSPINE> for NodeLabelSPINE {
    fn from(parameters: BasicSPINE) -> Self {
        Self { parameters }
    }
}

impl SPINEBased for NodeLabelSPINE {
    fn get_basic_spine(&self) -> &BasicSPINE {
        &self.parameters
    }
}

impl NodeTypesAnchorsGenerator for NodeLabelSPINE {}
impl AnchorsInferredNodeEmbeddingProperties for NodeLabelSPINE {
    fn get_model_name(&self) -> String {
        "Node-types-based SPINE".to_string()
    }

    fn get_embedding_size(&self, graph: &graph::Graph) -> Result<usize, String> {
        Ok(graph.get_number_of_node_types()? as usize)
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicAnchorsInferredNodeEmbedding {
        self.get_basic_spine().get_basic_inferred_node_embedding()
    }
}

impl AnchorsInferredNodeEmbeddingModel<{ AnchorTypes::NodeTypes }, {AnchorFeatureTypes::ShortestPaths}> for NodeLabelSPINE {}
