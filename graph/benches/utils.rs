use rand::Rng;

pub fn gen_random_vec(num: u64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let vals: Vec<u64> = (0..num).map(|_| rng.gen_range(0, num)).collect();
    let total: f64 = vals.iter().sum::<u64>() as f64;
    let weights = vals.iter().map(|x| *x as f64 / total).collect::<Vec<f64>>();
    //println!("{:?}", weights);
    weights
}

pub fn gen_random_u64_vec(num: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let vals: Vec<u64> = (0..num).map(|_| rng.gen_range(0, num)).collect();
    vals
}