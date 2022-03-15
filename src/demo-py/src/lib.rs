use pyo3::prelude::*;
use demo_lib;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn add(a: isize, b: isize) -> PyResult<isize> {
    let result = demo_lib::add(a, b);
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn demo_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
