// Goal: 
// Make the complex object computation fast. 
// - The complex object computation should be in the Rust domain (implemented with pure Rust objects). 
// - A `reduce` method operate on the Rust domain (input: json str list, output: Object str)
// - The Object str can be transformed to Python object in Python domain. 
// - When doing `|` operation in Python domain, `or` in Rust domain should be called.  
// TODO:
// 1. [ ] build pure rust objects 
//    - [X] Float
//    - [X] Int
//    - [X] Num 
//    - [X] Str
//    - [X] Non
//    - [X] Atomic
//    - [ ] Union 
//    - [ ] Optional
// 2. [ ] Let PyClass takes RustObjects as variable. 
//    - [X] Float
//    - [X] Int
//    - [X] Num 
//    - [X] Str
//    - [X] Non
//    - [X] Atomic
//    - [ ] Union 
//    - [ ] Optional
// 3. [ ] Let RustObject be able to be converted to a str ("Int()")
// 4. [ ] Implement methods on Rust objects and call them from the Python Object. 
use pyo3::prelude::*;
use pyo3::exceptions;
//use std::collections::HashSet;
use std::hash::{Hash, Hasher};
//use pyo3::types::PySet;
use pyo3::types::PyList;

////////////////// Non //////////////////
#[derive(Clone, Eq, PartialEq)]
struct RustNon {}
impl RustNon {
    fn new() -> RustNon {
        RustNon {}
    }
    fn repr(&self) -> String {
        format!("Non()")
    }
}
impl Hash for RustNon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state)
    }
}
impl IntoPy<PyObject> for RustNon {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}
#[derive(Clone)]
#[pyclass]
struct Non {
    rust_obj: RustNon,
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
////////////////// Float //////////////////
#[derive(Clone, Eq, PartialEq)]
struct RustFloat {}
impl RustFloat {
    fn new() -> RustFloat {
        RustFloat {}
    }
    fn repr(&self) -> String {
        format!("Float()")
    }
}
impl Hash for RustFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state)
    }
}
impl IntoPy<PyObject> for RustFloat {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}
#[derive(Clone)]
#[pyclass]
struct Float {
    rust_obj: RustFloat,
}
#[pymethods]
impl Float {
    #[new]
    fn new() -> Self {
        Float { rust_obj: RustFloat {} }
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}

////////////////// Int //////////////////
#[derive(Clone, Eq, PartialEq)]
struct RustInt {}
impl RustInt {
    fn new() -> RustInt {
        RustInt {}
    }
    fn repr(&self) -> String {
        format!("Int()")
    }
}
impl Hash for RustInt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state)
    }
}
impl IntoPy<PyObject> for RustInt {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}
#[derive(Clone)]
#[pyclass]
struct Int {
    rust_obj: RustInt,
}
#[pymethods]
impl Int {
    #[new]
    fn new() -> Self {
        let x = RustInt {};
        Int {rust_obj: x}
    }

    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
////////////////// Num //////////////////
#[derive(Clone, Eq, PartialEq)]
enum RustNum {
    Int(RustInt),
    Float(RustFloat)
}
impl RustNum {
    fn repr(&self) -> String {
        match self {
            RustNum::Int(int_val) => int_val.repr(),
            RustNum::Float(float_val) => float_val.repr(),
        }
    }
}
impl Hash for RustNum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RustNum::Int(i) => i.hash(state),
            RustNum::Float(f) => f.hash(state)
        }
    }
}
////////////////// String //////////////////
#[derive(Clone, Eq, PartialEq)]
struct RustStr {}
impl RustStr {
    fn new() -> RustStr {
        RustStr {}
    }
    fn repr(&self) -> String {
        format!("Str()")
    }
}
impl Hash for RustStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state)
    }
}
impl IntoPy<PyObject> for RustStr {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}
#[derive(Clone)]
#[pyclass]
struct Str {
    rust_obj: RustStr,
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
////////////////// Atomic //////////////////
#[derive(Clone, Eq, PartialEq)]
enum RustAtomic {
    Num(RustNum),
    Str(RustStr),
    Non(RustNon)
}
impl RustAtomic {
    fn repr(&self) -> String {
        match self {
            RustAtomic::Str(str_val) => format!("Atomic({})", str_val.repr()),
            RustAtomic::Num(num_val) => format!("Atomic({})", num_val.repr()),
            RustAtomic::Non(non_val) => format!("Atomic({})", non_val.repr()),
        }
    }
}
impl Hash for RustAtomic {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RustAtomic::Num(n) => n.hash(state),
            RustAtomic::Str(s) => s.hash(state),
            RustAtomic::Non(o) => o.hash(state)
        }
    }
}
impl IntoPy<PyObject> for RustAtomic {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}

#[derive(Clone)]
#[pyclass]
struct Atomic {
    rust_obj: RustAtomic,
}

#[pymethods]
impl Atomic {
    #[new]
    fn new(obj: &PyAny) -> PyResult<Self> {
        let rust_atomic = match (obj.extract::<Int>(), obj.extract::<Float>(), obj.extract::<Str>(), obj.extract::<Non>()) {
            (Ok(int), _, _, _) => RustAtomic::Num(RustNum::Int(int.rust_obj)),
            (_, Ok(float), _, _) => RustAtomic::Num(RustNum::Float(float.rust_obj)),
            (_, _, Ok(string), _) => RustAtomic::Str(string.rust_obj),
            (_, _, _, Ok(non)) => RustAtomic::Non(non.rust_obj),
            _ => return Err(exceptions::PyTypeError::new_err("Expect an Int, Float, Str or Non"))
        };
        Ok(Atomic { rust_obj: rust_atomic })
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}

//////////////// Union ////////////////////////
#[derive(Clone)]
struct RustUnion {
    content: Vec<RustAtomic>,
}
impl RustUnion {
    fn new(content: Vec<RustAtomic>) -> RustUnion {
        RustUnion {content: content}
    }
    fn repr(&self) -> String {
        let atom_reprs: Vec<String> = self.content.iter().map(|a| a.repr()).collect();
        format!("Union({{{}}})", atom_reprs.join(", "))
    }
}
impl IntoPy<PyObject> for RustUnion {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}
#[derive(Clone)]
#[pyclass]
struct Union {
    rust_obj: RustUnion,
}

#[pymethods]
impl Union {
    #[new]
    fn new(obj: &PyList) -> PyResult<Self> {
        let content = obj
            .into_iter()
            .map(|value| {
                value.extract::<Atomic>().map(|atom| atom.rust_obj).map_err(|err| err.into())
            })
            .collect::<Result<Vec<RustAtomic>, PyErr>>()?;
        Ok(Union { rust_obj: RustUnion {content: content} })
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}

#[pymodule]
fn rust_objs( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_class::<Int>()?;
    m.add_class::<Float>()?;
    m.add_class::<Str>()?;
    m.add_class::<Non>()?;
    m.add_class::<Atomic>()?;
    m.add_class::<Union>()?;
    return Ok( () );
}