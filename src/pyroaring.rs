use pyo3::prelude::*;
use roaring::RoaringBitmap;

#[pyclass(name = "Roaring")]
pub struct PyRoaring {
    inner: RoaringBitmap,
}

#[pymethods]
impl PyRoaring {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: RoaringBitmap::new(),
        }
    }

    pub fn len(&self) -> u64 {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn insert(&mut self, value: u32) -> bool {
        self.inner.insert(value)
    }

    pub fn push(&mut self, value: u32) -> bool {
        self.inner.push(value)
    }

    pub fn contains(&self, value: u32) -> bool {
        self.inner.contains(value)
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn insert_range(&mut self, start: u32, end: u32) -> u64 {
        self.inner.insert_range(start..=end)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}
