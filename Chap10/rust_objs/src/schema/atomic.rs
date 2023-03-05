use pyo3::prelude::*;
use pyo3::exceptions;
use super::num::{RustNum, Float, Int};
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
        let rust_atomic = match (obj.extract::<Int>(), obj.extract::<Float>(), obj.extract::<Str>(), obj.extract::<Non>(), obj.extract::<Bool>()) {
            (Ok(x), _, _, _, _) => RustAtomic::Num(RustNum::Int(x.rust_obj)),
            (_, Ok(x), _, _, _) => RustAtomic::Num(RustNum::Float(x.rust_obj)),
            (_, _, Ok(x), _, _) => RustAtomic::Str(x.rust_obj),
            (_, _, _, Ok(x), _) => RustAtomic::Non(x.rust_obj),
            (_, _, _, _, Ok(x)) => RustAtomic::Bool(x.rust_obj),
            _ => return Err(exceptions::PyTypeError::new_err("Expect an Int, Float, Str, Bool, or Non"))
        };
        Ok(Atomic { rust_obj: rust_atomic })
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
        Non { rust_obj: RustNon {} }
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
        Str { rust_obj: RustStr {} }
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
        Bool { rust_obj: RustBool {} }
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
        let str_atom = RustAtomic::Str(RustStr{});
        assert_eq!(str_atom.repr(), "Atomic(Str())");
        let boo_atom = RustAtomic::Bool(RustBool{});
        assert_eq!(boo_atom.repr(), "Atomic(Bool())");
        let non_atom = RustAtomic::Non(RustNon{});
        assert_eq!(non_atom.repr(), "Atomic(Non())");
        let int_atom = RustAtomic::Num(RustNum::Int(RustInt{}));
        assert_eq!(int_atom.repr(), "Atomic(Int())");
    }
}
