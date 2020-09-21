use super::*;
use graph::set_num_threads;

#[pymodule]
fn utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(set_num_threads))?;
    Ok(())
}