use pyo3::prelude::*;
use pyo3::exceptions;
use super::atomic::Atomic;
use super::record::Record;
use super::unions::Union;
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
        let rust_schema = match (obj.extract::<Atomic>(), obj.extract::<Array>(), obj.extract::<Record>(), obj.extract::<Union>()) {
            (Ok(atom), _, _, _) => RustJsonSchema::Atomic(atom.rust_obj),
            (_, Ok(arr), _, _) => RustJsonSchema::Array(arr.rust_obj),
            (_, _, Ok(rec), _) => RustJsonSchema::Record(rec.rust_obj),
            (_, _, _, Ok(uni)) => RustJsonSchema::Union(uni.rust_obj),
            _ => return Err(exceptions::PyTypeError::new_err("Expect an Atomic, Array, Record, or Union"))
        };
        Ok(Array { rust_obj: RustArray{content: Box::new(rust_schema)} })
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
    fn new(content: RustJsonSchema) -> RustArray {
        RustArray {content: Box::new(content)}
    }
    pub fn repr(&self) -> String {
        format!("Array({})", self.content.repr())
    }
}

use super::atomic::*;
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