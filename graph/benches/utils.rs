use rand::Rng;

pub fn gen_random_u64_vec(num: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let vals: Vec<u64> = (0..num).map(|_| rng.gen_range(0, num)).collect();
    vals
}

pub fn gen_random_f64_vec(num: u64) -> Vec<f64> {
    let vals: Vec<u64> = gen_random_u64_vec(num);
    let total: f64 = vals.iter().sum::<u64>() as f64;
    let weights = vals.iter().map(|x| *x as f64 / total).collect::<Vec<f64>>();
    weights
}

pub fn gen_random_f32_vec(num: u64) -> Vec<f32> {
    let vals: Vec<u64> = gen_random_u64_vec(num);
    let total: f32 = vals.iter().sum::<u64>() as f32;
    let weights = vals.iter().map(|x| *x as f32 / total).collect::<Vec<f32>>();
    weights
}

pub fn gen_random_usize_vec(num: usize, max: usize) -> Vec<usize> {
    // TODO! substitute with xorshiro
    let mut rng = rand::thread_rng();
    let vals: Vec<usize> = (0..num).map(|_| rng.gen_range(0, max)).collect();
    vals
}
