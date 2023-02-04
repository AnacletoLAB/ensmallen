extern crate graph;
use graph::isomorphism_iter::*;
use rayon::iter::plumbing::*;
use rayon::iter::*;

#[test]
fn test_isomorphism_iter() -> Result<(), String> {
    let hashes = vec![0, 3, 4, 1, 1, 1, 7, 8, 8, 9, 10, 10, 10, 10];
    let indices = (0..hashes.len() as u32).collect::<Vec<_>>();
    let nodes = (0..hashes.len() as u32).collect::<Vec<_>>();
    let truth: &[&[u32]] = &[&[3, 4, 5], &[7, 8], &[10, 11, 12, 13]];

    assert_eq!(
        truth,
        &EqualBucketsIter::new(&hashes, &indices, &nodes).collect::<Vec<_>>(),
    );

    let (low, high) = EqualBucketsIter::new(&hashes, &indices, &nodes).split();

    let truth_low: &[&[u32]] = &[&[3, 4, 5], &[7, 8]];
    let truth_high: &[&[u32]] = &[&[10, 11, 12, 13]];

    assert_eq!(truth_low, low.collect::<Vec<_>>());
    assert_eq!(truth_high, high.unwrap().collect::<Vec<_>>());

    assert_eq!(
        truth,
        &EqualBucketsParIter::new(hashes, indices, nodes).collect::<Vec<_>>(),
    );

    Ok(())
}
