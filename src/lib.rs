mod algo;
mod matrix;
mod pyalgo;
mod pymatrix;
mod pyroaring;

use pyalgo::{PyAlgo, PyAlgoType};
use pymatrix::PyMatrix;
use pyo3::prelude::*;
use pyroaring::PyRoaring;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from pybinding!".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<PyAlgo>()?;
    m.add_class::<PyAlgoType>()?;
    m.add_class::<PyMatrix>()?;
    m.add_class::<PyRoaring>()?;
    Ok(())
}
