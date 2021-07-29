use rayon::ThreadPool;

/// Returns a rayon thread pool handling Creation errors.
///
/// Getting a thread pool might return the error "Resource temporarly unavailable"
/// if the number of processes currently on the system is more than what set in
/// `ulimit -a`, which by default is 256851.
///
/// Moreover, we return an error if the number of selected CPUS is 1 or less.
/// Because the algorithms which use the pool requires at least 2 threads, and
/// we generally provide also an optimized single-thread version.
pub fn get_thread_pool() -> Result<(usize, ThreadPool), String> {
    let cpu_number = rayon::current_num_threads();

    if cpu_number <= 1 {
        return Err(concat!(
            "Cannot execute the parallel connected_components method when",
            " only a single CPU is made available.\n",
            "This might be an erroroneus configuration of the envionment",
            " variable RAYON_NUM_THREADS.\n",
            "If you really want to compute the connected components with",
            " these configurations, consider using random_spanning_arborescence_kruskal."
        )
        .to_string());
    }

    let mut attempts_left = 1_000_000;
    loop {
        match rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_number)
            .build()
        {
            Ok(thread_pool) => return Ok((cpu_number, thread_pool)),
            Err(internal_error) => {
                if attempts_left == 0 {
                    return Err(format!(
                        concat!(
                            "Unknown error while trying to allocate the thread pool for ",
                            "executing the parallel connected components algorithm.\n",
                            "In our experience this happens once in every 100 milions calls\n",
                            "The interal error is {:?}."
                        ),
                        internal_error
                    ));
                }
                let delay = std::time::Duration::from_millis(50);
                std::thread::sleep(delay);
                attempts_left -= 1;
            }
        }
    }
}
