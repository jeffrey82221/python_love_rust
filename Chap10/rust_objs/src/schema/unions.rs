use pyo3::prelude::*;
use pyo3::exceptions;
use pyo3::types::PySet;
use std::collections::HashSet;
use super::atomic::Atomic;
use super::array::Array;
use super::record::Record;
use super::top::RustJsonSchema;
use super::atomic::{RustNon, RustAtomic};
//////////////// Python Objs ////////////////////////
#[derive(Clone)]
#[pyclass]
pub struct Union {
    pub rust_obj: RustUnion,
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
        Ok(Union { rust_obj: RustUnion::new(content)})
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
#[derive(Clone)]
#[pyclass]
pub struct Optional {
    pub rust_obj: RustUnion,
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
        Ok(Optional { rust_obj: RustUnion::new(content)})
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
////////////////////// Rust Objs ///////////////////////////////

#[derive(Clone, Eq, PartialEq)]
pub struct RustUnion {
    pub content: HashSet<RustJsonSchema>,
}
impl RustUnion {
    fn new(content: HashSet<RustJsonSchema>) -> RustUnion {
        RustUnion {content: content}
    }
    pub fn repr(&self) -> String {
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

use super::atomic::*;
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_union() {
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let bool_atom = RustJsonSchema::Atomic(RustAtomic::Bool(RustBool{}));
        let bool_atom2 = bool_atom.clone();
        // Test 1: one element union
        let mut set = HashSet::new();
        set.insert(str_atom.clone());
        let u = RustUnion{ content: set};
        assert_eq!(u.content.len(), 1);
        assert_eq!(u.repr(), "Union({Atomic(Str())})");
        // Test 2: two element union
        let mut set = HashSet::new();
        set.insert(str_atom.clone());
        set.insert(bool_atom.clone());
        let u = RustUnion{ content: set};
        assert_eq!(u.content.len(), 2);
        assert_eq!(u.repr(), "Union({Atomic(Bool()), Atomic(Str())})");
        // Test 3: duplicate element union
        let mut set = HashSet::new();
        set.insert(str_atom.clone());
        set.insert(bool_atom.clone());
        set.insert(bool_atom2.clone());
        let u = RustUnion{ content: set};
        assert_eq!(u.content.len(), 2);
        assert_eq!(u.repr(), "Union({Atomic(Bool()), Atomic(Str())})");
        // Test 4: nested union
        let mut set = HashSet::new();
        set.insert(str_atom.clone());
        set.insert(bool_atom.clone());
        let u = RustJsonSchema::Union(RustUnion{ content: set});
        let mut set2 = HashSet::new();
        set2.insert(u.clone());
        set2.insert(str_atom.clone());
        set2.insert(bool_atom.clone());
        let u2 = RustUnion{ content: set2};
        assert_eq!(u2.content.len(), 3);
        assert_eq!(u2.repr(), "Union({Atomic(Bool()), Atomic(Str()), Union({Atomic(Bool()), Atomic(Str())})})");
    }
    #[test]
    fn test_optional() {
        let mut set = HashSet::new();
        let v1 = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let v2 = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        set.insert(v1.clone());
        set.insert(v2.clone());
        let u = RustUnion{ content: set};
        assert_eq!(u.content.len(), 2);
        assert_eq!(u.repr(), "Optional(Atomic(Str()))");
    }
}