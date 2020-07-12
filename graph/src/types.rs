use std::collections::HashMap;

// Types used to represent edges, nodes and their types.
pub type NodeT = usize;
pub type EdgeT = usize;
pub type WeightT = f64;
pub type ParamsT = f64;
pub type NodeTypeT = u16;
pub type EdgeTypeT = u16;

pub struct WalkParameters {
    length: usize,
    return_weight: Option<ParamsT>,
    explore_weight: Option<ParamsT>,
    change_node_type_weight: Option<ParamsT>,
    change_edge_type_weight: Option<ParamsT>,
    nodes_mapping: Option<HashMap<NodeT, NodeT>>
}