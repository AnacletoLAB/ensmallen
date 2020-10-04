use super::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
pub struct FromVecHarnessParams {
    edges: Vec<Result<(NodeT, NodeT, Option<EdgeTypeT>, Option<WeightT>), String>>,
    nodes: Option<Vocabulary<NodeT>>,
    edge_types_vocabulary: Option<VocabularyVec<EdgeTypeT>>,
    directed: bool,
    ignore_duplicated_edges: bool,
    verbose: bool,
    cached_edges_number: EdgeT
}

pub fn from_vec_harness(data: FromVecHarnessParams) -> Result<(), String> {
    let g = graph::Graph::from_integer_unsorted(
        data.edges.iter().cloned(),
        data.nodes,
        data.directed,
        data.ignore_duplicated_nodes,
        data.ignore_duplicated_edges,
        data.skip_self_loops
    )?;
    graph::test_utilities::default_test_suite(&g, false);

    Ok(())
}  
