use pyo3::prelude::*;


/// A Python module implemented in Rust.
#[pymodule]
fn fun(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn sum_as_string<'py>(a: usize, b: usize) -> String {
        let result = string_stuff::sum_as_string(a, b);
        result
    }
    Ok(())
}

mod string_stuff {

    pub fn sum_as_string(a: usize, b: usize) -> String {
        (a + b + b).to_string()
    }
}