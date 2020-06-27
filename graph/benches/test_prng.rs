#![feature(test)]
extern crate test;
use test::Bencher;

mod prng;
use prng::*;

const IERATIONS: usize = 10000;

#[bench]
fn test_gen_xorshift_random_float(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..IERATIONS {
            gen_xorshift_random_float();
        }
    });
}

// #[bench]
// fn test_sse_xorshift_random_float(b: &mut Bencher) {
//     b.iter(|| {
//         for _ in 0..IERATIONS {
//             sse_xorshift_random_float();
//         }
//     });

//     println!("{:?}", sse_xorshift_random_float());
//     println!("{:?}", sse_xorshift_random_float());
// }

#[bench]
fn test_gen_xorshiro256plus_mul(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..IERATIONS {
            xorshiro256plus_mul();
        }
    });
}

#[bench]
fn test_gen_xorshiro256plus(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..IERATIONS {
            xorshiro256plus();
        }
    });
}

#[bench]
fn test_gen_xorshiro256plus_no_mul(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..IERATIONS {
            xorshiro256plus_no_mul();
        }
    });
}