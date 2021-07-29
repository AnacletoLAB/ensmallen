use graph::Graph;
use shared::types::*;

pub fn test_remove_components(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    if graph.get_connected_components_number(verbose).0 > 1 {
        let without_selfloops = graph.drop_selfloops();

        assert_eq!(
            graph.get_connected_components_number(verbose),
            without_selfloops.get_connected_components_number(verbose),
            concat!(
                "We expected the graph to have the same components once we remove the selfloops.\n",
                "The report of the original graph is {:?}\n",
                "The report of the filtered graph is {:?}\n",
                "The edge node ids of the original graph are {:?}\n",
                "The edge node ids of the filtered graph are {:?}\n"
            ),
            graph.textual_report(),
            without_selfloops.textual_report(),
            graph.get_edge_node_ids(true),
            without_selfloops.get_edge_node_ids(true),
        );

        let single_component = graph.remove_components(None, None, None, None, Some(1), verbose);
        assert!(
            single_component.is_ok(),
            concat!(
                "Removing all the components except the first one returned an error.\n",
                "The error is:\n{:?}\nand the graph report is:\n{:?}"
            ),
            single_component,
            graph.textual_report()
        );
        let single_component_number = single_component
            .unwrap()
            .get_connected_components_number(verbose)
            .0;
        assert_eq!(
            single_component_number,
            1,
            concat!(
                "Removing all the components except the first one returned a graph ",
                "with {} components, which is not one.\nThe report of the graph is:{:?}\n"
            ),
            single_component_number,
            graph.textual_report()
        );

        let test = graph.remove_components(
            Some(vec![graph.nodes.unchecked_translate(0)]),
            None,
            None,
            None,
            None,
            verbose,
        )?;
        let without_selfloops = test.drop_selfloops();
        assert_eq!(
            without_selfloops.get_connected_components_number(verbose).0,
            1,
            concat!(
                "Expected number of components (1) is not matched!\n",
                "The report of the original graph is {:?}\n",
                "The report of the graph with only one component is {:?}\n",
                "The report of the graph without selfloops is {:?}\n",
            ),
            graph.textual_report(),
            test.textual_report(),
            without_selfloops.textual_report()
        );
        if let Ok(node_type_name) = graph.get_node_type_name_from_node_type_id(0) {
            assert!(graph
                .remove_components(
                    None,
                    Some(vec![Some(node_type_name)]),
                    None,
                    None,
                    None,
                    verbose
                )
                .is_ok());
        }
        if graph.has_unknown_node_types()? {
            let without_unknowns =
                graph.remove_components(None, Some(vec![None]), None, None, None, verbose);
            assert!(
                without_unknowns.is_ok(),
                "Could not remove components without node type None.\nThe error is {:?}\nThe graph report is {:?}",
                without_unknowns, graph.textual_report()
            );
        }
        if let Ok(edge_type_name) = graph.get_edge_type_name_from_edge_type_id(0) {
            assert!(graph
                .remove_components(
                    None,
                    None,
                    Some(vec![Some(edge_type_name)]),
                    None,
                    None,
                    verbose
                )
                .is_ok());
        }
        if graph.has_unknown_edge_types()? {
            assert!(graph
                .remove_components(None, None, Some(vec![None]), None, None, verbose)
                .is_ok());
        }
    } else {
        assert!(
            graph
                .remove_components(None, None, None, None, None, verbose)
                .is_ok(),
            "We expect it to be possible, now, to create empty graphs."
        );
    }

    Ok(())
}
