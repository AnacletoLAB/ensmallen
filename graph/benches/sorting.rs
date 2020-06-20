
use rayon::prelude::*;

pub fn parallel_unstable_sorting(values: &Vec<u64>) -> Vec<usize>{
    let mut pairs: Vec<(usize, &u64)> = values.par_iter().enumerate().collect();
    pairs.par_sort_unstable_by_key(|(_, &v)| v);
    let indices: Vec<usize> = pairs.par_iter().map(|(i, _)| *i).collect();
    indices
}


pub fn sequential_unstable_sorting(values: &Vec<u64>) -> Vec<usize>{
    let mut pairs: Vec<(usize, &u64)> = values.iter().enumerate().collect();
    pairs.sort_unstable_by_key(|(_, &v)| v);
    let indices: Vec<usize> = pairs.par_iter().map(|(i, _)| *i).collect();
    indices
}

pub fn parallel_stable_sorting(values: &Vec<u64>) -> Vec<usize>{
    let mut pairs: Vec<(usize, &u64)> = values.par_iter().enumerate().collect();
    pairs.par_sort_by_key(|(_, &v)| v);
    let indices: Vec<usize> = pairs.par_iter().map(|(i, _)| *i).collect();
    indices
}


pub fn sequential_stable_sorting(values: &Vec<u64>) -> Vec<usize>{
    let mut pairs: Vec<(usize, &u64)> = values.iter().enumerate().collect();
    pairs.sort_by_key(|(_, &v)| v);
    let indices: Vec<usize> = pairs.par_iter().map(|(i, _)| *i).collect();
    indices
}