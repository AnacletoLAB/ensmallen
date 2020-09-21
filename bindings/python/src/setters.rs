use super::*;
use graph::set_num_threads as rust_set_num_thread;

#[pymodule]
fn utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(set_num_threads))?;
    Ok(())
}

#[pyfunction]
#[text_signature = "(num_threads)"]
/// Set number of threads.
///
/// Arguments
/// ---------
///
/// num_threads: int,
///     Number of threads to use.
///
fn set_num_threads(num_threads: usize) {
    rust_set_num_thread(num_threads)
}
