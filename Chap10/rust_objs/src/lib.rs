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

#[derive(Clone)]
struct RustAtomic {
    content: i32   
}
impl RustAtomic {
    fn new(content: i32) -> RustAtomic {
        RustAtomic { content }
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
    fn new(content: i32) -> Self {
        let x = RustAtomic {content};
        Atomic { rust_obj: x }
    }

    fn __repr__(&self) -> String {
        format!("Atomic({})", self.rust_obj.content)
    }
}


#[pymodule]
fn rust_objs( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_class::<Atomic>()?;
    return Ok( () );
}