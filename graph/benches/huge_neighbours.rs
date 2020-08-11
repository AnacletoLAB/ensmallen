#![feature(test)]
extern crate test;
use test::Bencher;

extern crate graph;
use graph::FromCsvBuilder;
use graph::WalkWeights;
use vec_rand::xorshift::xorshift;

#[bench]
fn extract_edge(b: &mut Bencher) {
    let edge_path = "benches/data/test.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .build()
    .unwrap();

    println!("Degrees: {}", graph.degree(13));
    b.iter(|| {
        graph.extract_edge(13, &WalkWeights::default());
    });
}

#[bench]
fn extract_edge_normale(b: &mut Bencher) {
    let edge_path = "benches/data/test.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .build()
    .unwrap();

    println!("Degrees: {}", graph.degree(5));
    b.iter(|| {
        graph.extract_edge(5, &WalkWeights::default());
    });
}

#[bench]
fn extract_edge_random(b: &mut Bencher) {
    let edge_path = "benches/data/test.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .build()
    .unwrap();

    let mut seed : u64 = 0xbad5eed;
    b.iter(|| {
        let result = graph.extract_edge((seed % graph.get_edges_number() as u64) as usize, &WalkWeights::default());
        seed = xorshift(seed);
        result
    });
}


#[bench]
fn extract_nodes(b: &mut Bencher) {
    let edge_path = "benches/data/test.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .build()
    .unwrap();

    b.iter(|| {
        graph.extract_node(13, 1.0);
    });
}

#[bench]
fn extract_node_normale(b: &mut Bencher) {
    let edge_path = "benches/data/test.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .build()
    .unwrap();

    println!("Degrees: {}", graph.degree(5));
    b.iter(|| {
        graph.extract_node(5, 1.0);
    });
}

#[bench]
fn extract_node_random(b: &mut Bencher) {
    let edge_path = "benches/data/test.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .build()
    .unwrap();

    let mut seed : u64 = 0xbad5eed;
    b.iter(|| {
        let result = graph.extract_node((seed % graph.get_nodes_number() as u64) as usize, 1.0);
        seed = xorshift(seed);
        result
    });
}