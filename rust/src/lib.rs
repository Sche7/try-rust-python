use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
fn try_rust(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn fibonacci<'py>(n: i64) -> i64 {
        let result = _fibonacci::get_number(n);
        result
    }
    Ok(())
}


// Fibonacci implemented in Rust
mod _fibonacci {
    pub fn get_number(n: i64) -> i64 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        return get_number(n-1) + get_number(n-2);
    }
}