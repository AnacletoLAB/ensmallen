#![feature(stdarch)]

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

