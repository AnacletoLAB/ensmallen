extern crate graph;
use graph::{EdgeFileReader, Graph};
use std::collections::HashMap;

#[test]
/// Test that the components number is reasonable, this raised the wierd case in which:
/// singletons: false selfloops: false smallest: 1 biggest: 3, edges: [(0, 1), (2, 3), (4, 5)]
fn test_components_size() {
    let edges_reader = EdgeFileReader::new("tests/data/test_components.csv".to_string())
        .unwrap()
        .set_separator(Some(",".to_string()))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_header(Some(false));

    let g =
        Graph::from_sorted_csv(edges_reader, None, false, false, 6108, 242, "Graph".to_owned()).unwrap();

    // THIS IS NOT DETERMINISTIC
    for _ in 0..10_000 {
        let (components, _components_number, smallest, biggest) = g.connected_components(false).unwrap();
        assert!(biggest >= smallest, "smallest: {} biggest: {}", smallest, biggest);

        assert!(
            ! (
                smallest == 1
                &&
                (!g.has_singletons())
                &&
                (!g.has_selfloops())
            ),
            "singletons: {} selfloops: {} smallest: {} biggest: {}, edges: {:?}, components: {:?}", 
            g.has_singletons(), g.has_selfloops(), smallest, biggest, g.get_unique_edges_iter(false).collect::<Vec<(u32, u32)>>(), components
        );
    }

    let (components, number_of_components, smallest, biggest) = g.connected_components(false).unwrap();

    assert_eq!(components, [0, 0, 1, 1, 2, 2].to_vec());
    assert_eq!(number_of_components, 3);
    assert_eq!(smallest, 2); // the size of the smallest component
    assert_eq!(biggest, 2);  // the size of the biggest component

    let (number_of_components2, smallest2, biggest2) = g.connected_components_number(false);
    assert_eq!(number_of_components, number_of_components2, "There is a difference between the number of components returned by the connected_components method and the connected_components_number.");
    assert_eq!(smallest, smallest2, "There is a difference between the smallest returned by the connected_components method and the connected_components_number.");
    assert_eq!(biggest, biggest2, "There is a difference between the biggest returned by the connected_components method and the connected_components_number.");

    let mut components_size = HashMap::new();
    for component_index in &components {
        let counter = components_size.entry(*component_index).or_insert(0);
        *counter += 1;
    }

    assert_eq!(number_of_components as usize, components_size.len(), "The number of components is wrong!");
    assert_eq!(smallest, *components_size.values().min().unwrap(), "The smallest is wrong!");
    assert_eq!(biggest,  *components_size.values().max().unwrap(), "The biggest is wrong!");
}
