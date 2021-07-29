use graph::Graph;
use shared::types::*;

pub fn test_polygons(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    assert_eq!(
        graph
            .get_number_of_triangles_per_node(Some(false), None, verbose)
            .into_iter()
            .map(|triangles_number| triangles_number as EdgeT)
            .sum::<EdgeT>(),
        graph.get_number_of_triangles(Some(false), None, verbose)
    );
    Ok(())
}
