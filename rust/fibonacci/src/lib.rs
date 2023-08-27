use pyo3::prelude::*;

// Fibonacci implemented in Rust
#[pyfunction]
fn rust_fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return rust_fibonacci(n-1) + rust_fibonacci(n-2);
}


/// A Python module implemented in Rust.
#[pymodule]
fn fibonacci(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_fibonacci, m)?)?;
    Ok(())
}
