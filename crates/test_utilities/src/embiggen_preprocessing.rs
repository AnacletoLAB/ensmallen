use graph::Graph;
use shared::types::*;
use crate::utils::*;
use rayon::iter::ParallelIterator;

pub fn test_embiggen_preprocessing(graph: &mut Graph, verbose: Option<bool>) -> Result<()> {
    let walker = first_order_walker(&graph)?;
    if !graph.directed {
        let (terms_number, iterator) = graph.cooccurence_matrix(&walker, 3, verbose)?;
        assert_eq!(terms_number, iterator.count());

        let window_size = 3;
        let batch_size = 256;
        let data = graph
            .node2vec(&walker, batch_size, window_size)?
            .collect::<Vec<_>>();
        assert_eq!(
            data.len(),
            batch_size as usize
                * walker.iterations as usize
                * (walker.single_walk_parameters.walk_length as usize - window_size * 2)
        );
        for (context, _) in data.iter() {
            assert_eq!(context.len(), window_size * 2);
        }
    }
    if graph.has_edges() {
        graph
            .link_prediction_degrees(
                0,
                Some(256),
                Some(true),
                Some(0.3),
                Some(false),
                Some(10),
                Some(false),
                None,
            )
            .unwrap()
            .collect::<Vec<_>>();
        graph
            .get_edge_prediction_mini_batch(
                0,
                Some(256),
                Some(0.4),
                None,
                None,
                Some(false),
                Some(false),
                Some(10),
                Some(false),
                None,
            )
            .unwrap()
            .collect::<Vec<_>>();
    }

    Ok(())
}
