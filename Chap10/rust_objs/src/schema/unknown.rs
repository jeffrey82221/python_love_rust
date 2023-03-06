use pyo3::prelude::*;
#[derive(Clone, Copy)]
#[pyclass]
pub struct Unknown {
    pub rust_obj: RustUnknown,
}
#[pymethods]
impl Unknown {
    #[new]
    fn new() -> Self {
        Unknown { rust_obj: RustUnknown {} }
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RustUnknown {}
impl RustUnknown {
    pub fn new() -> RustUnknown {
        RustUnknown {}
    }
    pub fn repr(&self) -> String {
        format!("Unknown()")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_repr() {
        let unknown = RustUnknown::new();
        assert_eq!(unknown.repr(), "Unknown()");
    }
}