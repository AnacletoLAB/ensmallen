#![feature(test)]
extern crate test;
use test::{black_box, Bencher};

extern crate graph;
use graph::test_utilities::*;
use graph::NodeT;
use graph::Vocabulary;

fn populate_vocabulary(
    vocabulary_initial_size: usize,
    vocabulary_final_size: usize,
) -> Vocabulary<NodeT> {
    let mut my_vocabulary: Vocabulary<NodeT> = Vocabulary::with_capacity(vocabulary_initial_size);
    (0..vocabulary_final_size).for_each(|_| {
        my_vocabulary.insert(random_string(5));
    });
    my_vocabulary
}

#[bench]
fn bench_vocabulary_with_unknown_size(b: &mut Bencher) {
    let mut graph = load_cora();
    b.iter(|| {
        let _ = black_box(populate_vocabulary(0, 1000000));
    });
}

#[bench]
fn bench_vocabulary_with_known_size(b: &mut Bencher) {
    let mut graph = load_cora();
    b.iter(|| {
        let _ = black_box(populate_vocabulary(1000000, 1000000));
    });
}
