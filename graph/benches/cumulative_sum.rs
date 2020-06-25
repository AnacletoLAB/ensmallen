
pub fn naife_cumulative_f64_sum(random_vec: &Vec<f64>) -> Vec<f64> {
        let mut cumulative_sum: Vec<f64> = Vec::with_capacity(random_vec.len());
        let mut total_weight = 0f64;
        for w in random_vec {
            total_weight += w;
            cumulative_sum.push(total_weight.clone());
        }
        cumulative_sum
}

pub fn scan_cumulative_f64_sum(random_vec: &Vec<f64>) -> Vec<f64> {
    random_vec
    .iter()
    .scan(0f64, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    })
    .collect()
}

pub fn naife_cumulative_f32_sum(random_vec: &Vec<f32>) -> Vec<f32> {
    let mut cumulative_sum: Vec<f32> = Vec::with_capacity(random_vec.len());
    let mut total_weight = 0f32;
    for w in random_vec {
        total_weight += w;
        cumulative_sum.push(total_weight.clone());
    }
    cumulative_sum
}

pub fn scan_cumulative_f32_sum(random_vec: &Vec<f32>) -> Vec<f32> {
random_vec
.iter()
.scan(0f32, |acc, &x| {
    *acc = *acc + x;
    Some(*acc)
})
.collect()
}

pub fn unrolled_cumulative_f64_sum(random_vec: &Vec<f64>) -> Vec<f64> {
    let mut result = vec![0.0f64; 4 * ( random_vec.len() as f64 / 4.0).ceil() as usize];
    let mut offset = 0.0f64;

    for i in (0..random_vec.len()).step_by(4){
        let mut a = random_vec[i];
        let mut b = random_vec[i+1];
        let mut c = random_vec[i+2];
        let mut d = random_vec[i+3];

        d += c + b + a + offset;
        c += b + a + offset;
        b += a + offset;
        a += offset;

        result[i] = a;
        result[i+1] = b;
        result[i+2] = c;
        result[i+3] = d;

        offset = d;
    }

    for _ in 0..(random_vec.len() % 4){
        result.pop();
    }

    result
}
