use csr::*;
use rayon::prelude::*;

const NODES: u32 = 10;
const EDGES: &[(u32, u32)] = &[(0, 0), (0, 1), (1, 2), (1, 3), (2, 3), (3, 4), (4, 4)];

#[test]
fn test_edges_iter() -> Result<(), String> {
    let csrb = ConcurrentCSRBuilder::new(EDGES.len() as u64, NODES);

    EDGES.iter().enumerate().for_each(|(i, (src, dst))| {
        csrb.set(i as u64, *src, *dst);
    });

    let csr = csrb.build();

    assert_eq!(csr.get_number_of_nodes(), NODES);
    assert_eq!(csr.get_number_of_directed_edges(), EDGES.len() as u64);

    unsafe {
        for (i, (src, _)) in EDGES.iter().enumerate() {
            assert_eq!(csr.get_unchecked_source_node_id_from_edge_id(i as _), *src);
        }
    }

    csr.par_iter_directed_edge_node_ids()
        .enumerate()
        .for_each(|(i, (edge_id, src, dst))| {
            assert_eq!(edge_id, i as u64);
            assert_eq!(src, EDGES[i].0);
            assert_eq!(dst, EDGES[i].1);
        });

    Ok(())
}
