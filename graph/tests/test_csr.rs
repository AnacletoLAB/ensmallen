extern crate graph;
use graph::data_structures::{CSR, EdgesIter};
use rayon::iter::*;
use rayon::iter::plumbing::*;

#[test]
fn test_from_iter() -> Result<(), String> {
    let edges =  vec![
        (0, 1), 
        (0, 7), 
        (0, 8), 
        (1, 3), 
        (1, 8), 
        (2, 4), 
        (2, 9), 
        (4, 6), 
        (4, 9), 
        (5, 3),
        (6, 5),
        (6, 8),  
        (7, 2), 
        (7, 5), 
        (9, 3), 
    ];

    let g = unsafe{CSR::from_sorted_iter_unchecked(
        edges.iter().copied()
    )};
    for (i, src, dst) in g.iter_edge_node_ids(false) {
        assert_eq!(src, edges[i as usize].0);
        assert_eq!(dst, edges[i as usize].1);
    }

    let iter = EdgesIter::new(&g);
    assert_eq!(edges.len(), iter.len());
    for (i, src, dst) in iter {
        assert_eq!(src, edges[i as usize].0);
        assert_eq!(dst, edges[i as usize].1);
    }

    for (i, src, dst) in EdgesIter::new(&g).rev() {
        assert_eq!(src, edges[i as usize].0);
        assert_eq!(dst, edges[i as usize].1);
    }

    let par_edges =  g.par_iter_directed_edge_node_ids().collect::<Vec<_>>();
    assert_eq!(edges.len(), par_edges.len());
    for (i, src, dst) in par_edges.into_iter() {
        assert_eq!(src, edges[i as usize].0);
        assert_eq!(dst, edges[i as usize].1);
    }

    let idx = 5;
    let (lower_true, higher_true) = edges.split_at(idx);
    let (lower_iter, higher_iter) = EdgesIter::new(&g).split_at(idx);

    assert_eq!(higher_true.len(), higher_iter.len());
    for (i, src, dst) in higher_iter {
        assert_eq!(src, higher_true[i as usize - lower_true.len()].0);
        assert_eq!(dst, higher_true[i as usize - lower_true.len()].1);
    }

    assert_eq!(lower_true.len(), lower_iter.len());
    for (i, src, dst) in lower_iter {
        assert_eq!(src, lower_true[i as usize].0);
        assert_eq!(dst, lower_true[i as usize].1);
    }

    Ok(())
}
