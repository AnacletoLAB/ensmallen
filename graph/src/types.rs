use std::collections::HashSet;

// Types used to represent edges, nodes and their types.
pub type NodeT = usize;
pub type EdgeT = usize;
pub type WeightT = f64;
pub type ParamsT = f64;
pub type NodeTypeT = u16;
pub type EdgeTypeT = u16;

pub(crate) struct EdgeMetadata {
    pub(crate) edge_id: EdgeT,
    pub(crate) edge_types: HashSet<EdgeTypeT>,
}