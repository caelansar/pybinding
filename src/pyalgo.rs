use std::fmt;

use crate::algo::{Algo, AlgoType};
use pyo3::prelude::*;

#[pyclass(eq, name = "AlgoType")]
#[derive(PartialEq, Clone, Copy)]
pub enum PyAlgoType {
    Blake3,
    Default,
}

#[pyclass(name = "Algo")]
#[doc = "Rust Algo implementation."]
pub struct PyAlgo {
    inner: Algo,
}

#[pymethods]
impl PyAlgo {
    #[new]
    #[pyo3(signature = (typ=PyAlgoType::Default))]
    pub fn new(typ: PyAlgoType) -> Self {
        let algo = match typ {
            PyAlgoType::Blake3 => Algo::new(AlgoType::Blake3),
            PyAlgoType::Default => Algo::new(AlgoType::Default),
        };
        Self { inner: algo }
    }

    pub fn hash(&self, v: &str) -> String {
        self.inner.hash(v)
    }

    pub fn get_name(&self) -> &str {
        self.inner.get_name()
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for PyAlgo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.get_name())
    }
}
