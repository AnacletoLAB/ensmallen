#![feature(test)]
extern crate test;
use test::Bencher;

mod skipgram;
use skipgram::*;

mod utils;
use utils::*;

const NUMBER: usize = 10000;

#[bench]
fn test_naife_skipgram_preprocessing(b: &mut Bencher) {
    let random_vec = gen_random_usize_vec(NUMBER, NUMBER);
    b.iter(|| {
            naife_skipgram_preprocessing(&random_vec, NUMBER, 10, 1.0, true);
        }
    );
}