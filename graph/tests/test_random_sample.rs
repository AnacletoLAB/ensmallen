extern crate graph;
use graph::sample;
use rand::Rng;
use rayon::prelude::*;

const N_OF_ITERATIONS: usize = 10000;
const N_OF_CLASSES: usize = 1000;

pub fn gen_random_vec(number: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let vals: Vec<usize> = (0..number).map(|_| rng.gen_range(0, number)).collect();
    let total: f64 = vals.iter().sum::<usize>() as f64;
    let weights = vals.iter().map(|x| *x as f64 / total).collect::<Vec<f64>>();
    //println!("{:?}", weights);
    weights
}

#[test]
fn test_random_sample() {
    // TODO instead of sum of errors,
    //  use a wilcoxon test which currently isn't available in rust
    let mut random_weights = gen_random_vec(N_OF_CLASSES);

    let mut measurements: Vec<usize> = vec![0; N_OF_CLASSES];

    for _ in 0..N_OF_ITERATIONS {
        measurements[
            sample(& mut random_weights)
        ] += 1;
    }
    
    let normalized_measurements: Vec<f64> = measurements.par_iter()
        .map(
            |&x| x as f64 / N_OF_ITERATIONS as f64
        ).collect();

    let difference: f64 = random_weights.par_iter()
        .zip(normalized_measurements.par_iter())
        .map(
            |(a, b)|
                (a - b).abs()
        ).sum();

    println!("Difference on measurements: {}", difference);

    // assert!(difference < 0.1);
}
