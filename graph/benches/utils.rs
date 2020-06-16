use rand::Rng;
const NUMBER: u64 = 100000;

pub fn gen_random_vec() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let vals: Vec<u64> = (0..NUMBER).map(|_| rng.gen_range(0, NUMBER)).collect();
    let total: f64 = vals.iter().sum::<u64>() as f64;
    let weights = vals.iter().map(|x| *x as f64 / total).collect::<Vec<f64>>();
    //println!("{:?}", weights);
    weights
}