//! Fuzzing harness to test whether the cardinality estimation works as expected.
#![no_main]

use arbitrary::Arbitrary;
use csr::*;
use libfuzzer_sys::fuzz_target;
use rayon::prelude::*;

#[derive(Arbitrary, Debug)]
struct FuzzCase {
    // We use u8 so it cannot create
    // an object that is too big
    edges: Vec<(u8, u8)>,
    number_of_trailing_singletons: u8,
}

fuzz_target!(|data: FuzzCase| {
    let mut edges = data.edges.clone();

    let mut new_edges = edges.clone();

    for (src, dst) in edges.iter() {
        // If the edge list does not contain this
        // edge, add it to the new edge list
        if !edges.contains(&(*dst, *src)) {
            new_edges.push((*dst, *src));
        }
    }

    edges = new_edges;

    // sort edges
    edges.sort_unstable();

    // Identify the number of nodes, and adjust accordingly
    let number_of_nodes = (edges
        .iter()
        .copied()
        .map(|(src, dst)| src.max(dst))
        .max()
        .unwrap_or(0) as usize
        + 1)
        + data.number_of_trailing_singletons as usize;

    // Build the CSR
    let csrb = ConcurrentCSRBuilder::new(edges.len() as u64, number_of_nodes as u32);

    edges
        .iter()
        .copied()
        .enumerate()
        .for_each(|(i, (src, dst))| {
            csrb.set(i as u64, src as u32, dst as u32);
        });

    let lower_triangular_edges = edges
        .iter()
        .copied()
        .map(|(src, dst)| (src as u32, dst as u32))
        .filter(|(src, dst)| src >= dst)
        .collect::<Vec<_>>();

    let csr = csrb.build();

    // Check that the number of nodes is correct
    assert_eq!(csr.get_number_of_nodes(), number_of_nodes as u32);

    // Check that the number of edges is correct
    assert_eq!(csr.get_number_of_directed_edges(), edges.len() as u64);

    assert_eq!(
        lower_triangular_edges,
        EdgesIterLowerTriangular::new(&csr)
            .map(|(src, dst)| (src as _, dst as _))
            .collect::<Vec<_>>()
    );

    // Check that the directed edges are correct
    assert_eq!(
        csr.par_iter_directed_edge_node_ids()
            .map(|(_, src, dst)| (src, dst))
            .collect::<Vec<_>>(),
        edges
            .iter()
            .copied()
            .map(|(src, dst)| (src as u32, dst as u32))
            .collect::<Vec<_>>()
    );

    // Check that the lower_triangular edges are correct
    assert_eq!(
        csr.par_iter_lower_triangular_edge_node_ids()
            .collect::<Vec<_>>(),
        lower_triangular_edges,
    );
});
