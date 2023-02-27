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
//    - [X] Union (content a HashSet of RustAtomic)
//         FIXME: Union with Union as element does not make sense, 
//         but Union with Array, Atomic, or Record does make sense. 
//    - [X] Optional
//    - [X] Array 
//    - [ ] Record
// 2. [ ] Let PyClass takes RustObjects as variable. 
//    - [X] Float
//    - [X] Int
//    - [X] Num 
//    - [X] Str
//    - [X] Non
//    - [X] Atomic
//    - [X] Union (content a set of Atomic)
//    - [X] Optional
//    - [X] Array 
//    - [ ] Record
// 3. [ ] Implement | operation 
// 4. [ ] Let RustObject be able to be converted to a str ("Int()")
// 5. [ ] Implement methods on Rust objects and call them from the Python Object. 
// 6. [ ] UnitTest identical to the jsonschema python package.
use pyo3::prelude::*;
use pyo3::exceptions;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use pyo3::types::PySet;

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
        self.repr().hash(state)
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
//////////////////// Array ///////////////////////////
#[derive(Clone, Eq, PartialEq)]
struct RustArray {
    content: RustJsonSchema
}
impl RustArray {
    fn new(content: RustJsonSchema) -> RustArray {
        RustArray {content: content}
    }
    fn repr(&self) -> String {
        format!("Array({})", self.content.repr())
    }
}
impl IntoPy<PyObject> for RustArray {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}
#[derive(Clone)]
#[pyclass]
struct Array {
    rust_obj: RustArray,
}
#[pymethods]
impl Array {
    #[new]
    fn new(obj: &PyAny) -> PyResult<Self> {
        let rust_schema = match (obj.extract::<Atomic>(), obj.extract::<Array>(), obj.extract::<Union>()) {
            (Ok(atom), _, _) => RustJsonSchema::Atomic(atom.rust_obj),
            (_, Ok(arr), _) => RustJsonSchema::Array(Box::new(arr.rust_obj)),
            (_, _, Ok(uni)) => RustJsonSchema::Union(uni.rust_obj),
            _ => return Err(exceptions::PyTypeError::new_err("Expect an Atomic, Array, or Union"))
        };
        Ok(Array { rust_obj: RustArray{content: rust_schema} })
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
//////////////////// Record ///////////////////////////

//////////////////// JsonSchema ///////////////////////////
#[derive(Clone, Eq, PartialEq)]
enum RustJsonSchema {
    Atomic(RustAtomic),
    Array(Box<RustArray>),
    // Record
    Union(RustUnion) // Advance Json Schema
}
impl RustJsonSchema {
    fn repr(&self) -> String {
        match self {
            RustJsonSchema::Atomic(atom_val) => atom_val.repr(),
            RustJsonSchema::Union(union_val) => union_val.repr(),
            RustJsonSchema::Array(array_val) => array_val.repr(),
        }
    }
}
impl Hash for RustJsonSchema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.repr().hash(state)
    }
}
//////////////// Union + Optional (implement python interface only) ////////////////////////

#[derive(Clone, Eq, PartialEq)]
struct RustUnion {
    content: HashSet<RustJsonSchema>,
}
impl RustUnion {
    fn new(content: HashSet<RustJsonSchema>) -> RustUnion {
        RustUnion {content: content}
    }
    fn repr(&self) -> String {
        let mut has_non: u8 = 0;
        let mut total_cnt: u8 = 0;
        let mut reprs: Vec<String> = self.content.iter().map(|a| {
            let s = a.repr();
            if s == "Atomic(Non())" {
                has_non += 1;
            }
            total_cnt += 1;
            s
        }).collect();
        if (has_non > 0) & (total_cnt == 2) {
            reprs.retain(|x| *x != "Atomic(Non())");
            format!("Optional({})", reprs[0])
        } else {
            reprs.sort();
            format!("Union({{{}}})", reprs.join(", "))
        }
        
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
    fn new(obj: &PySet) -> PyResult<Self> {
        let mut content = HashSet::new();
        for value in obj.iter() {
            match (value.extract::<Atomic>(), value.extract::<Union>()){
                (Ok(a), _) => {
                    content.insert(RustJsonSchema::Atomic(a.rust_obj));
                },
                (_, Ok(u)) => {
                    content.insert(RustJsonSchema::Union(u.rust_obj));
                },
                _ => {
                    return Err(exceptions::PyTypeError::new_err("Expect an Atomic or Union"));
                }
            }
        }
        Ok(Union { rust_obj: RustUnion {content: content} })
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
#[derive(Clone)]
#[pyclass]
struct Optional {
    rust_obj: RustUnion,
}

#[pymethods]
impl Optional {
    #[new]
    fn new(obj: &Atomic) -> PyResult<Self> {
        let mut content = HashSet::new();
        content.insert(RustJsonSchema::Atomic(obj.rust_obj.clone()));
        content.insert(RustJsonSchema::Atomic(RustAtomic::Non(RustNon{})));
        Ok(Optional { rust_obj: RustUnion {content: content} })
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
    m.add_class::<Array>()?;
    m.add_class::<Union>()?;
    m.add_class::<Optional>()?;
    return Ok( () );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_atomic_hash() {
        let v1 = RustAtomic::Str(RustStr{});
        let v2 = RustAtomic::Non(RustNon{});
        let v3 = RustAtomic::Num(RustNum::Int(RustInt{}));
        let v4 = RustAtomic::Num(RustNum::Float(RustFloat{}));
        let mut set = HashSet::new();
        set.insert(v1.clone());
        assert_eq!(set.len(), 1);
        set.insert(v2.clone());
        assert_eq!(set.len(), 2);
        set.insert(v3.clone());
        assert_eq!(set.len(), 3);
        set.insert(v4.clone());
        assert_eq!(set.len(), 4);
        set.insert(v1.clone());
        assert_eq!(set.len(), 4);
        set.insert(v2.clone());
        assert_eq!(set.len(), 4);
        set.insert(v3.clone());
        assert_eq!(set.len(), 4);
        set.insert(v4.clone());
        assert_eq!(set.len(), 4);
    }
    #[test]
    fn test_union() {
        // Test 1: one element union
        let mut set = HashSet::new();
        let v1 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Int(RustInt{})));
        set.insert(v1.clone());
        let u = RustUnion{ content: set};
        assert_eq!(u.content.len(), 1);
        assert_eq!(u.repr(), "Union({Atomic(Int())})");
        // Test 2: two element union
        let mut set = HashSet::new();
        let v1 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Int(RustInt{})));
        let v2 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Float(RustFloat{})));
        set.insert(v1.clone());
        set.insert(v2.clone());
        let u = RustUnion{ content: set};
        assert_eq!(u.content.len(), 2);
        assert_eq!(u.repr(), "Union({Atomic(Float()), Atomic(Int())})");
        // Test 3: duplicate element union
        let mut set = HashSet::new();
        let v1 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Int(RustInt{})));
        let v2 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Float(RustFloat{})));
        let v3 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Float(RustFloat{})));
        set.insert(v1.clone());
        set.insert(v2.clone());
        set.insert(v3.clone());
        let u = RustUnion{ content: set};
        assert_eq!(u.content.len(), 2);
        assert_eq!(u.repr(), "Union({Atomic(Float()), Atomic(Int())})");
        // Test 4: nested union
        let mut set = HashSet::new();
        let v1 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Int(RustInt{})));
        let v2 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Float(RustFloat{})));
        set.insert(v1.clone());
        set.insert(v2.clone());
        let u = RustJsonSchema::Union(RustUnion{ content: set});
        let mut set2 = HashSet::new();
        let v1 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Int(RustInt{})));
        let v2 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Float(RustFloat{})));
        set2.insert(u.clone());
        set2.insert(v1.clone());
        set2.insert(v2.clone());
        let u2 = RustUnion{ content: set2};
        assert_eq!(u2.content.len(), 3);
        assert_eq!(u2.repr(), "Union({Atomic(Float()), Atomic(Int()), Union({Atomic(Float()), Atomic(Int())})})");
    }
    #[test]
    fn test_optional() {
        let mut set = HashSet::new();
        let v1 = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let v2 = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Int(RustInt{})));
        set.insert(v1.clone());
        set.insert(v2.clone());
        let u = RustUnion{ content: set};
        assert_eq!(u.content.len(), 2);
        assert_eq!(u.repr(), "Optional(Atomic(Int()))");
    }
    #[test]
    fn test_array() {
        let a = RustArray { content: RustJsonSchema::Atomic(RustAtomic::Non(RustNon{})) };
        assert_eq!(a.repr(), "Array(Atomic(Non()))");
        let b = RustArray { content: RustJsonSchema::Array(Box::new(a.clone()))};
        assert_eq!(b.repr(), "Array(Array(Atomic(Non())))");
    }
}