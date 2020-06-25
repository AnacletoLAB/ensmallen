use ::core::cmp::Ordering;

#[inline(always)]
pub fn extract_with_scan(weight: &Vec<f64>, rnd_val: f64) -> usize{
    let cumulative_sum: Vec<f64> = weight
    .iter()
    .scan(0f64, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    })
    .collect();

    let rnd: f64 = rnd_val * cumulative_sum[cumulative_sum.len() - 1];

    // Find the first item which has a weight *higher* than the chosen weight.
    match cumulative_sum.binary_search_by(
        |w|
            if *w <= rnd { 
                Ordering::Less 
            } else { 
                Ordering::Greater 
            }
        ){
            Ok(g) => g,
            Err(g) => g
        }
}

#[inline(always)]
pub fn extract_with_while(weight: &Vec<f64>, rnd_val: f64) -> usize{
    let mut cumulative_sum: Vec<f64> = Vec::with_capacity(weight.len());
    let mut total_weight = 0f64;
    for w in weight {
        total_weight += w;
        cumulative_sum.push(total_weight.clone());
    }

    let rnd: f64 = rnd_val * cumulative_sum[cumulative_sum.len() - 1];

    // Find the first item which has a weight *higher* than the chosen weight.
    match cumulative_sum.binary_search_by(
        |w|
            if *w <= rnd { 
                Ordering::Less 
            } else { 
                Ordering::Greater 
            }
        ) {
            Ok(g) => g,
            Err(g) => g
        }
}