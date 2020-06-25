#![feature(test)]
extern crate test;
use test::Bencher;

const NUMBER: u64 = 1000000;

mod utils;
use utils::*;

mod cumulative_sum;
use cumulative_sum::*;

mod cumulative_sum_sse_128_f32;
use cumulative_sum_sse_128_f32::*;

mod cumulative_sum_sse_128_f64;
use cumulative_sum_sse_128_f64::*;

#[bench]
fn test_naife_cumulative_f64_sum(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);

    let to_test = naife_cumulative_f64_sum(&random_vec);
    assert_eq!(to_test.len(), random_vec.len());

    b.iter(|| naife_cumulative_f64_sum(&random_vec));
}

#[bench]
fn test_scan_cumulative_f64_sum(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);
    
    let to_test = scan_cumulative_f64_sum(&random_vec);
    assert_eq!(to_test.len(), random_vec.len());

    b.iter(|| scan_cumulative_f64_sum(&random_vec));
}

#[bench]
fn test_naife_cumulative_f32_sum(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);

    let to_test = naife_cumulative_f32_sum(&random_vec);
    assert_eq!(to_test.len(), random_vec.len());

    b.iter(|| naife_cumulative_f32_sum(&random_vec));
}

#[bench]
fn test_scan_cumulative_f32_sum(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);
    
    let to_test = scan_cumulative_f32_sum(&random_vec);
    assert_eq!(to_test.len(), random_vec.len());

    b.iter(|| scan_cumulative_f32_sum(&random_vec));
}


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
      target_feature = "sse"))]
#[bench]
fn test_sse_128_f32_cumulative_sum(b: &mut Bencher) {
    let random_vec = gen_random_f32_vec(NUMBER);

    let to_test = sse_128_f32_cumulative_sum(&random_vec);
    assert_eq!(to_test.len(), random_vec.len());
    
    b.iter(|| sse_128_f32_cumulative_sum(&random_vec));

    let trusted = naife_cumulative_f32_sum(&random_vec);    

    assert_eq!(to_test.len(), trusted.len());

    for (a, b) in to_test.iter().zip(trusted.iter()){
        assert!((a - b).abs() < 0.0001);
    }
}


#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
      target_feature = "sse"))]
#[bench]
fn test_sse_128_f64_cumulative_sum(b: &mut Bencher) {
    let random_vec = gen_random_f64_vec(NUMBER);

    let to_test = sse_128_f64_cumulative_sum(&random_vec);
    assert_eq!(to_test.len(), random_vec.len());
    
    b.iter(|| sse_128_f64_cumulative_sum(&random_vec));

    let trusted = naife_cumulative_f64_sum(&random_vec);    

    assert_eq!(to_test.len(), trusted.len());

    for (a, b) in to_test.iter().zip(trusted.iter()){
        assert!((a - b).abs() < 0.0001);
    }
}