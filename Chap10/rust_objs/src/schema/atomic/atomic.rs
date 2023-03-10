use pyo3::prelude::*;
use super::num::RustNum;
use super::convert::py2rust;
////////////////// PyObjs ///////////////////
#[derive(Clone, Copy)]
#[pyclass]
pub struct Atomic {
    pub rust_obj: RustAtomic,
}
#[pymethods]
impl Atomic {
    #[new]
    fn new(obj: &PyAny) -> PyResult<Self> {
        Ok(Atomic { rust_obj: py2rust(obj) })
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
#[derive(Clone, Copy)]
#[pyclass]
pub struct Non {
    pub rust_obj: RustNon,
}
#[pymethods]
impl Non {
    #[new]
    fn new() -> Self {
        Non { rust_obj: RustNon::new() }
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
#[derive(Clone, Copy)]
#[pyclass]
pub struct Str {
    pub rust_obj: RustStr,
}
#[pymethods]
impl Str {
    #[new]
    fn new() -> Self {
        Str { rust_obj: RustStr::new() }
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
#[derive(Clone, Copy)]
#[pyclass]
pub struct Bool {
    pub rust_obj: RustBool,
}
#[pymethods]
impl Bool {
    #[new]
    fn new() -> Self {
        Bool { rust_obj: RustBool::new() }
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
////////////////// RustObjs ///////////////////
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum RustAtomic {
    Num(RustNum),
    Str(RustStr),
    Non(RustNon),
    Bool(RustBool)
}
impl RustAtomic {
    pub fn repr(&self) -> String {
        match self {
            RustAtomic::Str(val) => format!("Atomic({})", val.repr()),
            RustAtomic::Num(val) => format!("Atomic({})", val.repr()),
            RustAtomic::Non(val) => format!("Atomic({})", val.repr()),
            RustAtomic::Bool(val) => format!("Atomic({})", val.repr()),
        }
    }
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RustNon {}
impl RustNon {
    fn new() -> RustNon {
        RustNon {}
    }
    pub fn repr(&self) -> String {
        format!("Non()")
    }
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RustStr {}
impl RustStr {
    fn new() -> RustStr {
        RustStr {}
    }
    pub fn repr(&self) -> String {
        format!("Str()")
    }
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RustBool {}
impl RustBool {
    fn new() -> RustBool {
        RustBool {}
    }
    pub fn repr(&self) -> String {
        format!("Bool()")
    }
}
use super::num::*;
#[cfg(test)]
mod atomic_tests {
    use super::*;
    #[test]
    fn test_atomic() {
        let str_atom = RustAtomic::Str(RustStr::new());
        assert_eq!(str_atom.repr(), "Atomic(Str())");
        let boo_atom = RustAtomic::Bool(RustBool::new());
        assert_eq!(boo_atom.repr(), "Atomic(Bool())");
        let non_atom = RustAtomic::Non(RustNon::new());
        assert_eq!(non_atom.repr(), "Atomic(Non())");
        let int_atom = RustAtomic::Num(RustNum::Int(RustInt{}));
        assert_eq!(int_atom.repr(), "Atomic(Int())");
    }
}