use crate::*;

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

impl EmbeddingSize for NodeLabelWINE {
    fn get_embedding_size(&self, _graph: &graph::Graph) -> Result<usize, String> {
        Ok(self
            .parameters
            .get_basic_inferred_node_embedding()
            .get_embedding_size())
    }
}

impl NodeTypesLandmarkGenerator for NodeLabelWINE {}

impl ALPINE<{ LandmarkType::NodeTypes }, {LandmarkFeatureType::Windows}> for NodeLabelWINE {
    fn get_model_name(&self) -> String {
        "Node-types-based WINE".to_string()
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicALPINE {
        self.get_basic_wine().get_basic_inferred_node_embedding()
    }
}