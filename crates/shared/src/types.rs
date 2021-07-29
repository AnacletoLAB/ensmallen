//! Types used to represent edges, nodes and their types.

/// Type used to index the Nodes.
pub type NodeT = u32;
/// Type used to index the Node Types.
pub type NodeTypeT = u16;
/// Type used to index the Edges.
pub type EdgeT = u64;
/// Type used to index the Edge Types.
pub type EdgeTypeT = u16;
/// Type used for the weights of the edges.
pub type WeightT = f32;
/// Type used for the parameters of the walk such as the return weight (p),
/// and the explore weight (q).
pub type ParamsT = WeightT;
/// Type used to save contexts used for Skipgram and CBOW.
pub type Contexts = Vec<Vec<NodeT>>;
/// Type used to save a group of words indices.
pub type Words = Vec<NodeT>;
/// Type used to save the frequencies of words
pub type Frequencies = Vec<f64>;
/// Triple of edge data
pub type Triple = (NodeT, NodeT, Option<EdgeTypeT>);
/// Quadruple of edge data
pub type Quadruple = (NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>);
/// Quadrule of string edge data
pub type StringQuadruple = (String, String, Option<String>, WeightT);
/// Symbol reserved to unmapped nodes for algoritms such as connected components.
pub const NOT_PRESENT: NodeT = NodeT::MAX;

pub type Result<T> = std::result::Result<T, String>;