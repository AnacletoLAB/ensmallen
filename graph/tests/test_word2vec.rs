extern crate graph;
use graph::{NodeT, word2vec};
use rayon::prelude::*;

#[test]
fn test_word2vec() {
    let sequences = vec![vec![1, 2, 3, 4, 5, 6]];
    let result = word2vec(sequences.into_par_iter(), 2)
        .unwrap()
        .collect::<Vec<(Vec<NodeT>, NodeT)>>();
    assert_eq!(result, vec![
        (vec![1, 2, 4, 5], 3), 
        (vec![2, 3, 5, 6], 4),
    ]);
}
