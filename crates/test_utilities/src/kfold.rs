use graph::Graph;
use shared::types::*;
use crate::holdouts::*;

pub fn test_kfold(graph: &mut Graph, _verbose: Option<bool>) -> Result<()> {
    let k = 3;
    for i in 0..k {
        let (train, test) = graph.kfold(k, i, None, None, None)?;
        assert!(
            test.get_edges_number() <= (graph.get_edges_number() / k) + 1,
            concat!(
                "Check that test kfolds respect size bound has failed!\n",
                "The value of k is {}.\n",
                "The report of the original graph is:\n{:?}\n",
                "The report of the train graph is:\n{:?}\n",
                "The report of the test graph is:\n{:?}\n",
                "We expect that the test graph has at most {} edges but it has {}.\n",
                "The holdout index is {}.\n",
            ),
            k,
            graph.textual_report(),
            train.textual_report(),
            test.textual_report(),
            (graph.get_edges_number() / k) + 1,
            test.get_edges_number(),
            i
        );
        default_holdout_test_suite(graph, &train, &test)?;
    }

    if let Ok(edge_t) = graph.get_edge_type_name_from_edge_type_id(0) {
        for i in 0..k {
            let (train, test) = graph.kfold(k, i, Some(vec![Some(edge_t.clone())]), None, None)?;
            default_holdout_test_suite(graph, &train, &test)?;
        }
    }

    Ok(())
}
