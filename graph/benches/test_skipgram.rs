#![feature(test)]
extern crate test;
use test::Bencher;
use rand::Rng;
mod skipgram;
use skipgram::*;

const NUMBER: usize = 10000;

pub fn gen_random_usize_vec(num: usize, max: usize) -> Vec<usize> {
    // TODO! substitute with xorshiro
    let mut rng = rand::thread_rng();
    let vals: Vec<usize> = (0..num).map(|_| rng.gen_range(0, max)).collect();
    vals
}


#[bench]
fn test_naife_skipgram_preprocessing(b: &mut Bencher) {
    let random_vec = gen_random_usize_vec(NUMBER, NUMBER);
    b.iter(|| {
            naife_skipgram_preprocessing(&random_vec, NUMBER, 10, 1.0, true);
        }
    );
}