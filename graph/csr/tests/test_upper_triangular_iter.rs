use csr::*;
use rayon::{iter::plumbing::Producer, prelude::*};

const NODES: u32 = 5;
const EDGES: &[(u32, u32)] = &[
    (0, 0),
    (0, 1),
    //
    (1, 0),
    (1, 2),
    (1, 3),
    //
    (2, 1),
    (2, 3),
    //
    (3, 1),
    (3, 2),
    (3, 4),
    //
    (4, 3),
];
const EDGES_UND: &[(u32, u32)] = &[(0, 0), (0, 1), (1, 2), (1, 3), (2, 3), (3, 4)];

#[test]
fn test_upper_triangular_edges_iter() -> Result<(), String> {
    let csrb = ConcurrentCSRBuilder::new(EDGES.len() as u64, NODES);

    EDGES.iter().enumerate().for_each(|(i, (src, dst))| {
        csrb.set(i as u64, *src, *dst);
    });

    let csr = csrb.build();

    assert_eq!(csr.get_number_of_nodes(), NODES);
    assert_eq!(csr.get_number_of_directed_edges(), EDGES.len() as u64);

    let par_iter = csr.par_iter_upper_triangular_edge_node_ids();

    assert_eq!(par_iter.len(), EDGES_UND.len());

    assert_eq!(
        EDGES_UND,
        EdgesIterUpperTriangular::new(&csr).collect::<Vec<_>>()
    );
    //assert_eq!(
    //    EDGES_UND.iter().rev().copied().collect::<Vec<_>>(),
    //    EdgesIterUpperTriangular::new(&csr)
    //        .rev()
    //        .collect::<Vec<_>>()
    //);

    let iter = EdgesIterUpperTriangular::new(&csr);

    let idx = 4;

    let (lower, higher) = iter.split_at(idx);
    assert_eq!(EDGES_UND[..idx], lower.collect::<Vec<_>>(),);
    assert_eq!(EDGES_UND[idx..], higher.collect::<Vec<_>>(),);

    let pedges = par_iter.enumerate().collect::<Vec<_>>();
    pedges.into_iter().for_each(|(i, (src, dst))| {
        assert_eq!(src, EDGES_UND[i].0);
        assert_eq!(dst, EDGES_UND[i].1);
    });

    Ok(())
}
