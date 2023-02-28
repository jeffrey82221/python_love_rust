// Goal: 
// Make the complex object computation fast. 
// - The complex object computation should be in the Rust domain (implemented with pure Rust objects). 
// - A `reduce` method operate on the Rust domain (input: json str list, output: Object str)
// - The Object str can be transformed to Python object in Python domain. 
// - When doing `|` operation in Python domain, `or` in Rust domain should be called.  
// REFACTOR TODO:
// - [ ] Seperate RustObjs and PythonObjs
// FEAT TODO:
// 1. [ ] Build pure rust objects 
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
//         - [X] Basic Representation 
//         - [?] UniformRecord: Uniform Representation (Merge all schemas)
//         - [ ] Record(): Dynamic Representation 1 (show fields and their schemas with fields ordered by occurrence): Record()
//         - [ ] Union({Record()}): Dynamic Representation 2 (show only the field combination and their schemas as different Records):  Union({Record(xx), Record(xx)})
//         - [ ] Auto decide representation by length. 
// 2. [X] Let PyClass takes RustObjects as variable. 
//    - [X] Float
//    - [X] Int
//    - [X] Num 
//    - [X] Str
//    - [X] Non
//    - [X] Atomic
//    - [X] Union (content a set of Atomic)
//    - [X] Optional
//    - [X] Array 
//    - [X] Record 
// 3. [X] Implement | operation 
//    - [X] Atomic + Atomic -> Atomic / Union
//    - [X] Atomic + Union -> Union
//    - [X] Atomic + Array -> Union 
//    - [X] Atomic + Record -> Union 
//    - [X] Array + Atomic -> Union
//    - [X] Array + Union -> Union
//    - [X] Array + Array -> Array[xx]
//    - [X] Array + Record -> Union 
//    - [X] Record + Atomic -> Union
//    - [X] Record + Array -> Union
//    - [X] Record + Record -> Record
//    - [X] Record + Union -> Union 
//    - [X] Union + Atomic -> Union
//    - [X] Union + Union -> Union 
//    - [X] Union + Array -> Union
//    - [X] Union + Record -> Union
// 4. [ ] Let RustObject be able to be converted to a str ("Int()")
// 5. [ ] Implement methods on Rust objects and call them from the Python Object. 
// 6. [ ] UnitTest identical to the jsonschema python package.
use pyo3::prelude::*;
use pyo3::exceptions;
use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use pyo3::types::PySet;
use pyo3::types::PyDict;

////////////////// Non //////////////////
#[derive(Clone, Copy, Eq, PartialEq)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Eq, PartialEq)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Eq, PartialEq)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Eq, PartialEq)]
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
#[derive(Clone, Copy, Eq, PartialEq)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Eq, PartialEq)]
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

impl IntoPy<PyObject> for RustAtomic {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}

#[derive(Clone, Copy)]
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
    content: Box<RustJsonSchema>
}
impl RustArray {
    fn new(content: RustJsonSchema) -> RustArray {
        RustArray {content: Box::new(content)}
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
//////////////////// Record ///////////////////////////
#[derive(Clone, Eq, PartialEq)]
struct RustRecord {
    // A field schema recorder:
    field_schema: HashMap<String, RustJsonSchema>,
    // A field combination counter: 
    field_comb_counter: HashMap<String, u32>,
    // A field counter:
    field_counter: HashMap<String, u32>
}
impl RustRecord {
    fn new(field_schema: HashMap<String, RustJsonSchema>) -> RustRecord {
        let mut field_comb_counter = HashMap::new();
        let keys: HashSet<String> = field_schema.keys().cloned().collect();
        let mut key_vec: Vec<String> = keys.into_iter().collect();
        key_vec.sort();
        field_comb_counter.insert(key_vec.join(", "), 1);
        let mut field_counter = HashMap::new();
        for key in field_schema.keys() {
            field_counter.insert(key.clone(), 1);
        }
        RustRecord {
            field_schema: field_schema,
            field_comb_counter: field_comb_counter,
            field_counter: field_counter
        }
    }
    fn repr(&self) -> String {
        let mut strings: Vec<String> = self.field_schema.iter()
            .map(|(key, value)| format!("\"{}\": {}", key, value.repr()))
            .collect();
        strings.sort();
        format!("Record({{{}}})", strings.join(", "))
    }
}
impl IntoPy<PyObject> for RustRecord {
    fn into_py(self, py: Python) -> PyObject {
        py.None()
    }
}
#[derive(Clone)]
#[pyclass]
struct Record {
    rust_obj: RustRecord,
}

#[pymethods]
impl Record {
    #[new]
    fn new(obj: &PyDict) -> PyResult<Self> {
        let mut content = HashMap::new();
        for (key, value) in obj.iter() {
            let key_str: String = key.extract().unwrap();
            let rust_schema = match (value.extract::<Atomic>(), value.extract::<Array>(), value.extract::<Record>(), value.extract::<Union>()) {
                (Ok(atom), _, _, _) => RustJsonSchema::Atomic(atom.rust_obj),
                (_, Ok(arr), _, _) => RustJsonSchema::Array(arr.rust_obj),
                (_, _, Ok(rec), _) => RustJsonSchema::Record(rec.rust_obj),
                (_, _, _, Ok(uni)) => RustJsonSchema::Union(uni.rust_obj),
                _ => return Err(exceptions::PyTypeError::new_err("Expect Atomic, Array, Record, or Union"))
            };
            content.insert(key_str, rust_schema);
        }
        Ok(Record { rust_obj: RustRecord::new(content)})
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
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
        let mut cnt: u32 = 0;
        for value in obj.iter() {
            cnt += 1;
            match (value.extract::<Atomic>(), value.extract::<Array>(), value.extract::<Record>()){
                (Ok(a), _, _) => {
                    content.insert(RustJsonSchema::Atomic(a.rust_obj));
                },
                (_, Ok(a), _) => {
                    content.insert(RustJsonSchema::Array(a.rust_obj));
                },
                (_, _, Ok(a)) => {
                    content.insert(RustJsonSchema::Record(a.rust_obj));
                },
                _ => {
                    return Err(exceptions::PyTypeError::new_err("Expect an Atomic, Array, or Record"));
                }
            }
        }
        if cnt < 2 {
            panic!("# of content of Union should >= 2")
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
    fn new(obj: &PyAny) -> PyResult<Self> {
        let mut content = HashSet::new();
        let rust_schema = match (obj.extract::<Atomic>(), obj.extract::<Array>(), obj.extract::<Record>()){
            (Ok(a), _, _) => RustJsonSchema::Atomic(a.rust_obj.clone()),
            (_, Ok(a), _) => RustJsonSchema::Array(a.rust_obj.clone()),
            (_, _, Ok(a)) => RustJsonSchema::Record(a.rust_obj.clone()),
            _ => return Err(exceptions::PyTypeError::new_err("Expect an Atomic, Array, or Record"))
        };
        content.insert(rust_schema);
        content.insert(RustJsonSchema::Atomic(RustAtomic::Non(RustNon{})));
        Ok(Optional { rust_obj: RustUnion {content: content} })
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
//////////////////// JsonSchema ///////////////////////////
#[derive(Clone, Eq, PartialEq)]
enum RustJsonSchema {
    Atomic(RustAtomic),
    Array(RustArray),
    Record(RustRecord),
    Union(RustUnion) // Advance Json Schema
}
impl RustJsonSchema {
    fn repr(&self) -> String {
        match self {
            RustJsonSchema::Atomic(atom_val) => atom_val.repr(),
            RustJsonSchema::Array(array_val) => array_val.repr(),
            RustJsonSchema::Record(record_val) => record_val.repr(),
            RustJsonSchema::Union(union_val) => union_val.repr(),
        }
    }
    fn merge(self, other:RustJsonSchema) -> RustJsonSchema {
        match self {
            RustJsonSchema::Atomic(ref l) => {
                match other {
                    RustJsonSchema::Atomic(_r) => {
                        if l.repr() == _r.repr() {
                            RustJsonSchema::Atomic(l.clone())
                        } else {
                            let mut content = HashSet::new();
                            content.insert(self.clone());
                            content.insert(other.clone());
                            RustJsonSchema::Union(RustUnion {content: content})
                        }
                    },
                    RustJsonSchema::Array(_) => {                        
                        let mut content = HashSet::new();
                        content.insert(self.clone());
                        content.insert(other.clone());
                        RustJsonSchema::Union(RustUnion {content: content})                        
                    },
                    RustJsonSchema::Record(_) => {                        
                        let mut content = HashSet::new();
                        content.insert(self.clone());
                        content.insert(other.clone());
                        RustJsonSchema::Union(RustUnion {content: content})                        
                    },
                    RustJsonSchema::Union(_r) => {
                        let mut content = HashSet::new();
                        content.extend(_r.content);
                        content.insert(self.clone());
                        RustJsonSchema::Union(RustUnion {content: content})                        
                    },
                }
            },
            RustJsonSchema::Array(ref l) => {
                match other {
                    RustJsonSchema::Atomic(_) => {
                        other.merge(self)
                    },
                    RustJsonSchema::Array(_r) => {
                        RustJsonSchema::Array( RustArray { content: Box::new(l.content.clone().merge(*_r.clone().content))} )
                    },
                    RustJsonSchema::Record(_) => {                        
                        let mut content = HashSet::new();
                        content.insert(self.clone());
                        content.insert(other.clone());
                        RustJsonSchema::Union(RustUnion {content: content})                        
                    },
                    RustJsonSchema::Union(_r) => {
                        let mut content = HashSet::new();
                        let mut has_array: u8 = 0;
                        for jsonschema in _r.content.iter() {
                            match jsonschema {
                                RustJsonSchema::Atomic(_) => {
                                    content.insert(jsonschema.clone());
                                },
                                RustJsonSchema::Array(_) => {
                                    content.insert(self.clone().merge(jsonschema.clone()));
                                    has_array += 1;
                                },
                                RustJsonSchema::Record(_) => {
                                    content.insert(jsonschema.clone());
                                },
                                RustJsonSchema::Union(_u) => {
                                    content.extend(_u.content.clone());
                                }
                            }
                        }
                        if has_array == 0 {
                            content.insert(self.clone());
                        }
                        RustJsonSchema::Union(RustUnion {content: content})      
                    },
                }
            },
            RustJsonSchema::Record(ref l) => {
                match other {
                    RustJsonSchema::Atomic(_) => {
                        other.merge(self)
                    },
                    RustJsonSchema::Array(_) => {
                        other.merge(self)
                    },
                    RustJsonSchema::Record(_r) => {
                        // A field schema recorder:
                        let mut field_schema = l.field_schema.clone();
                        for (key, r_schema) in _r.field_schema.iter() {
                            match l.field_schema.get(key) {
                                Some(l_schema) => {
                                    let merged_schema = l_schema.clone().merge(r_schema.clone());
                                    field_schema.insert(key.to_string(), merged_schema);
                                },
                                None => {
                                    field_schema.insert(key.to_string(), r_schema.clone());
                                }
                            }
                        }
                        let mut field_counter = l.field_counter.clone();
                        for (key, r_cnt) in _r.field_counter.iter() {
                            match l.field_counter.get(key) {
                                Some(l_cnt) => {
                                    field_counter.insert(key.to_string(), r_cnt.clone() + l_cnt.clone());
                                },
                                None => {
                                    field_counter.insert(key.to_string(), r_cnt.clone());
                                }
                            }
                        }
                        let mut field_comb_counter = l.field_comb_counter.clone();
                        for (key, r_cnt) in _r.field_comb_counter.iter() {
                            match l.field_comb_counter.get(key) {
                                Some(l_cnt) => {
                                    field_comb_counter.insert(key.to_string(), r_cnt.clone() + l_cnt.clone());
                                },
                                None => {
                                    field_comb_counter.insert(key.to_string(), r_cnt.clone());
                                }
                            }
                        }
                        RustJsonSchema::Record(RustRecord{
                            field_schema: field_schema,
                            field_comb_counter: field_comb_counter,
                            field_counter: field_counter
                        })
                    },
                    RustJsonSchema::Union(_r) => {
                        let mut content = HashSet::new();
                        let mut has_record: u8 = 0;
                        for jsonschema in _r.content.iter() {
                            match jsonschema {
                                RustJsonSchema::Atomic(_) => {
                                    content.insert(jsonschema.clone());
                                },
                                RustJsonSchema::Array(_) => {
                                    content.insert(jsonschema.clone());
                                },
                                RustJsonSchema::Record(_) => {
                                    content.insert(self.clone().merge(jsonschema.clone()));
                                    has_record += 1;
                                },
                                RustJsonSchema::Union(_u) => {
                                    content.extend(_u.content.clone());
                                }
                            }
                        }
                        if has_record == 0 {
                            content.insert(self.clone());
                        }
                        RustJsonSchema::Union(RustUnion {content: content})   
                    },
                }
            },
            RustJsonSchema::Union(ref l) => {
                match other {
                    RustJsonSchema::Atomic(_) => {
                        other.merge(self)
                    },
                    RustJsonSchema::Array(_) => {
                        other.merge(self)
                    },
                    RustJsonSchema::Record(_) => {
                        other.merge(self)
                    },
                    RustJsonSchema::Union(_r) => {
                        let mut content = HashSet::new();
                        content.extend(l.content.clone());
                        content.extend(_r.content.clone());
                        RustJsonSchema::Union(RustUnion {content: content})                        
                    },
                }
            }
        }
    }
}
impl Hash for RustJsonSchema {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.repr().hash(state)
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
    m.add_class::<Record>()?;
    m.add_class::<Union>()?;
    m.add_class::<Optional>()?;
    return Ok( () );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
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
        let a = RustArray { content: Box::new(RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}))) };
        assert_eq!(a.repr(), "Array(Atomic(Non()))");
        let b = RustArray { content: Box::new(RustJsonSchema::Array(a.clone()))};
        assert_eq!(b.repr(), "Array(Array(Atomic(Non())))");
    }
    #[test]
    fn test_merge() {
        // Atomic | Atomic (1)
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        assert_eq!(non_atom.merge(str_atom).repr(), "Optional(Atomic(Str()))");
        // Atomic | Atomic (2)
        let str_atom1 = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let str_atom2 = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        assert_eq!(str_atom1.merge(str_atom2).repr(), "Atomic(Str())");
        // Atomic | Array
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        assert_eq!(str_atom.merge(array).repr(), "Union({Array(Atomic(Str())), Atomic(Str())})");
        // Atomic | Union (1)
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        let uni = array.merge(non_atom);
        assert_eq!(str_atom.merge(uni).repr(), "Union({Array(Atomic(Str())), Atomic(Non()), Atomic(Str())})");
        // Atomic | Union (2)
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let str2_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        let uni = str_atom.merge(array.merge(non_atom));
        assert_eq!(str2_atom.merge(uni).repr(), "Union({Array(Atomic(Str())), Atomic(Non()), Atomic(Str())})");
        // Array | Atomic 
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        assert_eq!(array.merge(str_atom).repr(), "Union({Array(Atomic(Str())), Atomic(Str())})");
        // Array | Array 
        let str_array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        let non_array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Non(RustNon{})))});
        assert_eq!(str_array.merge(non_array).repr(), "Array(Optional(Atomic(Str())))");
        // Array | Union (1)
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        let uni = str_atom.merge(non_atom);
        assert_eq!(array.merge(uni).repr(), "Union({Array(Atomic(Str())), Atomic(Non()), Atomic(Str())})");
        // Array | Union (2)
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let non_array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Non(RustNon{})))});
        let str_array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        let uni = str_atom.merge(non_atom).merge(non_array);
        assert_eq!(str_array.merge(uni).repr(), "Union({Array(Optional(Atomic(Str()))), Atomic(Non()), Atomic(Str())})");
        // Union | Atomic
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        let uni = array.merge(non_atom);
        assert_eq!(uni.merge(str_atom).repr(), "Union({Array(Atomic(Str())), Atomic(Non()), Atomic(Str())})");
        // Union | Array
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let non_array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Non(RustNon{})))});
        let str_array = RustJsonSchema::Array(RustArray{ content: Box::new(RustJsonSchema::Atomic(RustAtomic::Str(RustStr{})))});
        let uni = str_atom.merge(non_atom).merge(non_array);
        assert_eq!(uni.merge(str_array).repr(), "Union({Array(Optional(Atomic(Str()))), Atomic(Non()), Atomic(Str())})");
        // Union | Union (1)
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let uni1 = str_atom.merge(non_atom);
        let int_atom = RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Int(RustInt{})));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let uni2 = int_atom.merge(non_atom);
        assert_eq!(uni1.merge(uni2).repr(), "Union({Atomic(Int()), Atomic(Non()), Atomic(Str())})");
        // Union | Union (2)
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let uni1 = str_atom.merge(non_atom);
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let uni2 = str_atom.merge(non_atom);
        assert_eq!(uni1.merge(uni2).repr(), "Optional(Atomic(Str()))");
        // Record | Union 
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom);
        map.insert("apple".to_owned(), str_atom);
        let rr1 = RustJsonSchema::Record(RustRecord::new(map));
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let uni1 = str_atom.merge(non_atom).merge(rr1.clone());
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let mut map = HashMap::new();
        map.insert("can".to_owned(), str_atom);
        map.insert("banana".to_owned(), non_atom);
        let rr2 = RustJsonSchema::Record(RustRecord::new(map));
        assert_eq!(rr2.merge(uni1).repr(), "Union({Atomic(Non()), Atomic(Str()), Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non()), \"can\": Atomic(Str())})})");
        // Record | Record
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom);
        map.insert("apple".to_owned(), str_atom);
        let rr1 = RustJsonSchema::Record(RustRecord::new(map));
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let mut map = HashMap::new();
        map.insert("can".to_owned(), str_atom);
        map.insert("banana".to_owned(), non_atom);
        let rr2 = RustJsonSchema::Record(RustRecord::new(map));
        assert_eq!(rr1.clone().merge(rr2.clone()).repr(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non()), \"can\": Atomic(Str())})");
        let r = rr1.clone().merge(rr2.clone());
        match r {
            RustJsonSchema::Record(record) => {
                assert_eq!(record.field_counter.len(), 3);
                match record.field_counter.get("apple") {
                    Some(cnt) => {
                        assert_eq!(cnt.to_owned(), 1);
                    },
                    None => {
                        panic!();
                    }
                }
                match record.field_counter.get("banana") {
                    Some(cnt) => {
                        assert_eq!(cnt.to_owned(), 2);
                    },
                    None => {
                        panic!();
                    }
                }
                match record.field_counter.get("can") {
                    Some(cnt) => {
                        assert_eq!(cnt.to_owned(), 1);
                    },
                    None => {
                        panic!();
                    }
                }
                assert_eq!(record.field_comb_counter.len(), 2);
                match record.field_comb_counter.get("apple, banana") {
                    Some(cnt) => {
                        assert_eq!(cnt.to_owned(), 1);
                    },
                    None => {
                        panic!();
                    }
                }
                match record.field_comb_counter.get("banana, can") {
                    Some(cnt) => {
                        assert_eq!(cnt.to_owned(), 1);
                    },
                    None => {
                        panic!();
                    }
                }
            }
            _ => {
                panic!();
            }
        }
    }

    #[test]
    fn test_record() {
        // Test constructor from RustRecord
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom);
        map.insert("apple".to_owned(), str_atom);
        let rr = RustRecord::new(map);
        assert_eq!(rr.repr(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})");
        assert_eq!(rr.field_comb_counter.keys().len(), 1);
        assert_eq!(rr.field_counter.keys().len(), 2);
        // Test constructor from RustJsonSchema
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom);
        map.insert("apple".to_owned(), str_atom);
        let rr = RustJsonSchema::Record(RustRecord::new(map));
        assert_eq!(rr.repr(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})");
    }
}