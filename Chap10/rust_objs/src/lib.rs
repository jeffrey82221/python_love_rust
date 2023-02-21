// Goal: 
// Make the complex object computation fast. 
// - The complex object computation should be in the Rust domain (implemented with pure Rust objects). 
// - A `reduce` method operate on the Rust domain (input: json str list, output: Object str)
// - The Object str can be transformed to Python object in Python domain. 
// - When doing `|` operation in Python domain, `or` in Rust domain should be called.  
// TODO:
// 1. [ ] build pure rust objects (RustJsonSchema)
// 2. [ ] Let Point pyclass takes RustPoint as variable. 
// 3. [ ] Let RustPoint be able to be converted to a str ("Point(x, y)")
// 4. [ ] Implement methods on RustPoint and call them from the Python Point method. 
use pyo3::prelude::*;
////////////////// Float //////////////////
#[derive(Clone)]
struct RustFloat {}
impl RustFloat {
    fn new() -> RustFloat {
        RustFloat {}
    }
    fn repr(&self) -> String {
        format!("Float()")
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
    #[pyo3(get)]
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
#[derive(Clone)]
struct RustInt {}
impl RustInt {
    fn new() -> RustInt {
        RustInt {}
    }
    fn repr(&self) -> String {
        format!("Int()")
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
    #[pyo3(get)]
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

////////////////// Atomic //////////////////
#[derive(Clone)]
struct RustAtomic {
    content: RustInt   
}
impl RustAtomic {
    fn new(i: RustInt) -> RustAtomic {
        RustAtomic { content: i}
    } 
    fn repr(&self) -> String {
        format!("Atomic({})", self.content.repr())
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
    #[pyo3(get)]
    rust_obj: RustAtomic,
}

#[pymethods]
impl Atomic {
    #[new]
    fn new(i: Int) -> Self {
        let x = RustAtomic {content: i.rust_obj};
        Atomic { rust_obj: x }
    }

    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}


#[pymodule]
fn rust_objs( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_class::<Int>()?;
    m.add_class::<Float>()?;
    m.add_class::<Atomic>()?;
    return Ok( () );
}