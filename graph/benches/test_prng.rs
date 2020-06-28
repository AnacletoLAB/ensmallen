#![feature(test, asm)]
extern crate test;
use test::Bencher;

extern crate rand;
use rand::Rng;

mod prng;
use prng::*;

mod avx2_xorshift;
use avx2_xorshift::*;

mod avx2_xorshiro256plus;
use avx2_xorshiro256plus::*;

const IERATIONS: usize = 10000;


#[bench]
fn test_thread_rng(b: &mut Bencher) {
    let mut rng = rand::thread_rng();
    b.iter(|| {
        rng.gen_range(0, 10000)
    });
}

#[bench]
fn test_xorshift(b: &mut Bencher) {
    b.iter(|| {
        xorshift() % 10000
    });
}

#[bench]
fn test_gen_xorshiro256plus(b: &mut Bencher) {
    b.iter(|| {
        xorshiro256plus() % 10000
    });
}


#[bench]
fn test_avx2_xorshift(b: &mut Bencher) {
    b.iter(|| {
        avx2_xorshift()
    });

    println!("{:?}", avx2_xorshift());
}

#[bench]
fn test_ax2_xorshift_assembly(b: &mut Bencher) {
    let mut seed: [u64; 4] = [0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed];
    b.iter(|| {
        ax2_xorshift_assembly(& mut seed)
    });

    println!("{:?}", seed);
}

#[bench]
fn test_ax2_ss4_xorshift_assembly(b: &mut Bencher) {
    let mut seed: [u64; 16] = [
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed
    ];
    b.iter(|| {
        ax2_ss4_xorshift_assembly(& mut seed)
    });

    println!("{:?}", seed);
}

#[bench]
fn test_ax2_ss8_xorshift_assembly(b: &mut Bencher) {
    let mut seed: [u64; 32] = [
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed
    ];
    b.iter(|| {
        ax2_ss8_xorshift_assembly(& mut seed)
    });

    println!("{:?}", seed);
}

#[bench]
fn test_ax2_xorshiro_assembly(b: &mut Bencher) {
    let mut seed: [u64; 20] = [
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0
    ];
    b.iter(|| {
        ax2_xorshiro_assembly(& mut seed)
    });

    println!("{:?}", seed[4]);
}

#[bench]
fn test_ax2_ss4_xorshiro_assembly(b: &mut Bencher) {
    let mut seed: [u64; 80] = [
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
        0x1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed, 0,
    ];
    b.iter(|| {
        ax2_ss4_xorshiro_assembly(& mut seed)
    });

    println!("{:?}", seed[4]);
}

#[bench]
fn test_gen_xorshift_random_float(b: &mut Bencher) {
    b.iter(|| {
        gen_xorshift_random_float()
    });
}

#[bench]
fn test_gen_xorshiro256plus_no_mul(b: &mut Bencher) {
    b.iter(|| {
        xorshiro256plus_no_mul()
    });
}

#[bench]
fn test_gen_xorshiro256plus_mul(b: &mut Bencher) {
    b.iter(|| {
        xorshiro256plus_mul()
    });
}