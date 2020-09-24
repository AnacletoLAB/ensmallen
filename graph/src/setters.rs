/// Set the number of Rayon threads to given one.
/// 
/// # Arguments
/// 
/// * num_threads: usize - Maximum number of threads to be used by Rayon.
pub fn set_num_threads(num_threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
}