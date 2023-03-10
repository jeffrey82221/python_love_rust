use pyo3::prelude::*;
use super::convert::py2rust;
use super::top::RustJsonSchema;
#[derive(Clone)]
#[pyclass]
pub struct Array {
    pub rust_obj: RustArray,
}
#[pymethods]
impl Array {
    #[new]
    fn new(obj: &PyAny) -> PyResult<Self> {
        Ok(Array { rust_obj: RustArray{content: Box::new(py2rust(obj))} })
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct RustArray {
    pub content: Box<RustJsonSchema>
}
impl RustArray {
    pub fn new(content: RustJsonSchema) -> RustArray {
        RustArray {content: Box::new(content)}
    }
    pub fn repr(&self) -> String {
        format!("Array({})", self.content.repr())
    }
}

use super::atomic::atomic::*;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array() {
        let a = RustArray { content: Box::new(RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}))) };
        assert_eq!(a.repr(), "Array(Atomic(Non()))");
        let b = RustArray { content: Box::new(RustJsonSchema::Array(a.clone()))};
        assert_eq!(b.repr(), "Array(Array(Atomic(Non())))");
    }
}