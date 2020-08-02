#![feature(test)]
extern crate test;
use test::Bencher;

extern crate graph;
use graph::FromCsvBuilder;

const NUMBER: usize = 100000;

mod sorting;
use sorting::*;
mod utils;
use utils::*;

#[bench]
fn extract_random_nodes(b: &mut Bencher) {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .set_weights("weight", Some(1.0))
    .load_nodes_csv(
        node_path,
        "id",
        "category",
        Some("biolink:NamedThing"),
        None,
        None,
    )
    .unwrap()
    .build()
    .unwrap();

    let mut seed = 0xbad5eed;
    b.iter(|| {
        let result = graph.extract_random_nodes(NUMBER, seed);
        seed += 1337;
        result
    });
}

#[bench]
fn extract_random_nodes_par(b: &mut Bencher) {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .set_weights("weight", Some(1.0))
    .load_nodes_csv(
        node_path,
        "id",
        "category",
        Some("biolink:NamedThing"),
        None,
        None,
    )
    .unwrap()
    .build()
    .unwrap();

    let mut seed = 0xbad5eed;
    b.iter(|| {
        let result = graph.extract_random_nodes_par(NUMBER, seed, None);
        seed += 1337;
        result
    });
}


#[bench]
fn extract_random_edges(b: &mut Bencher) {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .set_weights("weight", Some(1.0))
    .load_nodes_csv(
        node_path,
        "id",
        "category",
        Some("biolink:NamedThing"),
        None,
        None,
    )
    .unwrap()
    .build()
    .unwrap();

    let mut seed = 0xbad5eed;
    b.iter(|| {
        let result = graph.extract_random_edges(NUMBER, seed);
        seed += 1337;
        result
    });
}

#[bench]
fn extract_random_edges_par(b: &mut Bencher) {
    let edge_path = "tests/data/het_graph_edges.tsv";
    let node_path = "tests/data/het_graph_nodes.tsv";
    let graph = FromCsvBuilder::new(edge_path, "subject", "object", false, None)
    .unwrap()
    .set_weights("weight", Some(1.0))
    .load_nodes_csv(
        node_path,
        "id",
        "category",
        Some("biolink:NamedThing"),
        None,
        None,
    )
    .unwrap()
    .build()
    .unwrap();

    let mut seed = 0xbad5eed;
    b.iter(|| {
        let result = graph.extract_random_edges_par(NUMBER, seed, None);
        seed += 1337;
        result
    });
}
