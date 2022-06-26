#![feature(test)]
extern crate test;
use test::{black_box, Bencher};

extern crate graph;
use graph::test_utilities::*;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x << 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

#[bench]
fn bench_has_edge_cora_enable(b: &mut Bencher) {
    let mut cora = load_cora();
    cora.enable(None, None, None, None).unwrap();
    let mut rng = Rng(0x8c2b_781f_2866_90fd);
    let random_vals = (0..1_000).map(|_| {
        let src = rng.next() as u32 % cora.get_number_of_nodes();
        let dst = rng.next() as u32 % cora.get_number_of_nodes();
        (src, dst)
    }).collect::<Vec<_>>();
    b.iter(|| {
        let mut counter = 0;
        for (src, dst) in random_vals.iter() {
            if cora.has_edge_from_node_ids(black_box(*src), black_box(*dst)) {
                counter += 1;
            }
        }
        counter
    });
}

#[bench]
fn bench_has_edge_cora(b: &mut Bencher) {
    let cora = load_cora();
    let mut rng = Rng(0x8c2b_781f_2866_90fd);
    let random_vals = (0..1_000).map(|_| {
        let src = rng.next() as u32 % cora.get_number_of_nodes();
        let dst = rng.next() as u32 % cora.get_number_of_nodes();
        (src, dst)
    }).collect::<Vec<_>>();
    b.iter(|| {
        let mut counter = 0;
        for (src, dst) in random_vals.iter() {
            if cora.has_edge_from_node_ids(black_box(*src), black_box(*dst)) {
                counter += 1;
            }
        }
        counter
    });
}

#[bench]
fn bench_has_edge_ppi_enable(b: &mut Bencher) {
    let mut cora = load_ppi(
        true,
        true,
        true,
        false,
        false,
        false,
    );
    cora.enable(None, None, None, None).unwrap();
    let mut rng = Rng(0x8c2b_781f_2866_90fd);
    let random_vals = (0..1_000).map(|_| {
        let src = rng.next() as u32 % cora.get_number_of_nodes();
        let dst = rng.next() as u32 % cora.get_number_of_nodes();
        (src, dst)
    }).collect::<Vec<_>>();
    b.iter(|| {
        let mut counter = 0;
        for (src, dst) in random_vals.iter() {
            if cora.has_edge_from_node_ids(black_box(*src), black_box(*dst)) {
                counter += 1;
            }
        }
        counter
    });
}

#[bench]
fn bench_has_edge_ppi(b: &mut Bencher) {
    let cora = load_ppi(
        true,
        true,
        true,
        false,
        false,
        false,
    );
    let mut rng = Rng(0x8c2b_781f_2866_90fd);
    let random_vals = (0..1_000).map(|_| {
        let src = rng.next() as u32 % cora.get_number_of_nodes();
        let dst = rng.next() as u32 % cora.get_number_of_nodes();
        (src, dst)
    }).collect::<Vec<_>>();
    b.iter(|| {
        let mut counter = 0;
        for (src, dst) in random_vals.iter() {
            if cora.has_edge_from_node_ids(black_box(*src), black_box(*dst)) {
                counter += 1;
            }
        }
        counter
    });
}

