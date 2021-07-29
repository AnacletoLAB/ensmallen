use graph::Graph;
use shared::types::*;
use crate::utils::*;


pub fn test_graph_filter(graph: &Graph, _verbose: Option<bool>) -> Result<()> {
    let unfiltered = graph
        .filter_from_ids(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None,
        )
        .unwrap();
    assert_eq!(&unfiltered, graph);
    assert!(graph
        .filter_from_names(
            None,
            Some(vec![NONEXISTENT]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .is_err());
    for node_name in graph.iter_node_names().take(10) {
        // The following test should remove ONLY the given node dijkstra
        let graph_without_given_name_result = graph.filter_from_names(
            None,
            Some(vec![node_name.as_str()]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(graph_without_given_name_result.is_ok());
        let graph_without_given_id = graph_without_given_name_result.unwrap();
        assert_eq!(
            graph_without_given_id.has_nodes(),
            graph.get_nodes_number() > 1
        );
        assert!(!graph_without_given_id.has_node_name(node_name.as_str()));

        // The following test should keep ONLY the given node name
        let graph_with_given_name_result = graph.filter_from_names(
            Some(vec![node_name.as_str()]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(
            graph_with_given_name_result.is_ok(),
            concat!(
                "Graph built with filter from names was expected to be okay, ",
                "but produced the following error message: {:?}."
            ),
            graph_with_given_name_result
        );
        let graph_with_given_node_name = graph_with_given_name_result.unwrap();
        assert_eq!(
            graph_with_given_node_name.has_selfloops(),
            graph.has_edge_from_node_names(node_name.as_ref(), node_name.as_ref())
        );
        assert_eq!(
            graph_with_given_node_name.has_edges(),
            graph_with_given_node_name.has_selfloops()
        );
        assert_eq!(graph_with_given_node_name.get_nodes_number(), 1);
        assert!(graph_with_given_node_name.has_node_name(node_name.as_str()));
    }

    for node_type_name in graph.iter_unique_node_type_names()?.take(10) {
        // The following test should remove ONLY the given node name
        let graph_without_given_node_type_name_result = graph.filter_from_names(
            None,
            None,
            None,
            None,
            None,
            Some(vec![Some(node_type_name.clone())]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(graph_without_given_node_type_name_result.is_ok());
        let graph_without_given_node_type_name = graph_without_given_node_type_name_result.unwrap();
        if graph.get_node_types_number()? > 1 && !graph.has_multilabel_node_types()? {
            assert!(graph_without_given_node_type_name.has_node_types());
            assert!(graph_without_given_node_type_name.has_nodes());
        }
        assert!(!graph_without_given_node_type_name.has_node_type_name(node_type_name.as_str()));
    }

    Ok(())
}
