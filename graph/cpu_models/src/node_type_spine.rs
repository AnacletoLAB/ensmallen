use crate::*;

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

impl EmbeddingSize for NodeLabelSPINE {
    fn get_embedding_size(&self, graph: &graph::Graph) -> Result<usize, String> {
        graph.get_number_of_node_types().map(|x| x as usize)
    }
}

impl NodeTypesLandmarkGenerator for NodeLabelSPINE {}

impl ALPINE<{ LandmarkType::NodeTypes }, { LandmarkFeatureType::ShortestPaths }>
    for NodeLabelSPINE
{
    fn get_model_name(&self) -> String {
        "Node-types-based SPINE".to_string()
    }

    fn get_basic_inferred_node_embedding(&self) -> &crate::BasicALPINE {
        self.get_basic_spine().get_basic_inferred_node_embedding()
    }
}
