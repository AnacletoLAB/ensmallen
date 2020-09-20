use super::*;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
pub struct FromVecHarnessParams {
    edges: Vec<Result<(String, String, Option<String>, Option<WeightT>), String>>,
    nodes: Option<Vec<Result<(String, Option<String>), String>>>,
    directed: bool,
    ignore_duplicated_nodes: bool,
    ignore_duplicated_edges: bool,
    skip_self_loops: bool,
}

pub fn from_vec_harness(data: FromVecHarnessParams) -> Result<(), String> {
    let g = graph::Graph::new(
        data.edges.iter().cloned(),
        if let Some(dn) = &data.nodes {
            Some(dn.iter().cloned())
        } else {
            None
        },
        data.directed,
        data.ignore_duplicated_nodes,
        data.ignore_duplicated_edges,
        data.skip_self_loops
    )?;
    graph::test_utilities::default_test_suite(&g, false);

    Ok(())
}  
