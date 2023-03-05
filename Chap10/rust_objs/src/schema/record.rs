/*
TODO:
let record fields ordered by occurrence 
- [ ] repr_co_occurence
- [ ] repr_normal
- [ ] repr_uniform
*/
use pyo3::prelude::*;
use pyo3::exceptions;
use pyo3::types::{PySet, PyDict};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use super::top::RustJsonSchema;
use super::atomic::Atomic;
use super::array::Array;
use super::unions::Union;
////////////// PyObjs ///////////////////
#[derive(Clone)]
#[pyclass]
pub struct Record {
    pub rust_obj: RustRecord,
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

#[derive(Clone)]
#[pyclass]
pub struct UniformRecord {
    pub rust_obj: RustRecord,
}

#[pymethods]
impl UniformRecord {
    #[new]
    fn new(field_set: FieldSet, value: &PyAny) -> PyResult<UniformRecord> {
        let mut content = HashMap::new();
        let rust_schema = match (value.extract::<Atomic>(), value.extract::<Array>(), value.extract::<Record>(), value.extract::<Union>()) {
            (Ok(atom), _, _, _) => RustJsonSchema::Atomic(atom.rust_obj),
            (_, Ok(arr), _, _) => RustJsonSchema::Array(arr.rust_obj),
            (_, _, Ok(rec), _) => RustJsonSchema::Record(rec.rust_obj),
            (_, _, _, Ok(uni)) => RustJsonSchema::Union(uni.rust_obj),
            _ => return Err(exceptions::PyTypeError::new_err("Expect Atomic, Array, Record, or Union"))
        };
        for key_str in field_set.rust_obj.content.iter() {
            content.insert(key_str.clone(), rust_schema.clone());
        }
        Ok(UniformRecord { rust_obj: RustRecord::new(content)})
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}

#[derive(Clone)]
#[pyclass]
pub struct FieldSet {
    pub rust_obj: RustFieldSet,
}

#[pymethods]
impl FieldSet {
    #[new]
    fn new(obj: &PySet) -> PyResult<Self> {
        let mut fields = HashSet::new();
        for item in obj.iter() {
            let field = item.to_string();
            fields.insert(field);
        }
        Ok(FieldSet { rust_obj: RustFieldSet {content: fields} })
    }
    fn __repr__(&self) -> String {
        self.rust_obj.repr()
    }
}
///////////////////// Rust Objs //////////////////////////////
const TOL_BY_NUM_CHAR_IN_LINE: usize = 30;
#[derive(Clone, Eq, PartialEq)]
pub struct RustRecord {
    pub field_schema: HashMap<String, RustJsonSchema>,
    pub field_comb_counter: HashMap<RustFieldSet, u32>,
    // TODO: change u32 to bool
    pub field_counter: HashMap<String, u32>
    // TODO: change u32 to bool
}
impl RustRecord {
    pub fn new(field_schema: HashMap<String, RustJsonSchema>) -> RustRecord {
        let mut field_comb_counter = HashMap::new();
        let keys: HashSet<String> = field_schema.keys().cloned().collect();
        field_comb_counter.insert(RustFieldSet {content: keys}, 1);
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
    pub fn repr(&self) -> String {
        // 1. [X] If united schema is not Union({xxx}), show repr_uniform_record. 
        // 2. [X] If repr_co_occurrence and repr_normal is very close, use repr_co_occurrence. 
        let reduced_schemas = reduce(self.field_schema.values().cloned().collect());
        match reduced_schemas {
            RustJsonSchema::Union(u) => {
                let repr_n = self.repr_normal();
                if self.field_comb_counter.len() == 1 {
                    repr_n
                } else {
                    let repr_c = self.repr_co_occurence();
                    if repr_c.len() - repr_n.len() < TOL_BY_NUM_CHAR_IN_LINE {
                        repr_c
                    } else {
                        repr_n
                    }
                }
            },
            _ => {
                self.repr_uniform(reduced_schemas)
            }
        }
        
    }
    fn repr_uniform(&self, json_schema: RustJsonSchema) -> String {
        let keys: HashSet<String> = self.field_schema.keys().cloned().collect();
        let field_set = RustFieldSet {content: keys};
        format!("UniformRecord({}, {})", field_set.repr(), json_schema.repr())
    }
    fn repr_normal(&self) -> String {
        // This is the representation with medium amount of information
        let keys_set: HashSet<String> = self.field_schema.keys().cloned().collect();
        self.compose_record_str(keys_set)
    }
    fn repr_co_occurence(&self) -> String {
        // This is the representation with largest amount of information
        let mut record_strings: Vec<String> = self.field_comb_counter.keys().into_iter()
            .map(|fieldset| self.compose_record_str(fieldset.content.clone()))
            .collect();
        record_strings.sort();
        format!("Union({{{}}})", record_strings.join(", "))
    }
    fn compose_record_str(&self, fieldset: HashSet<String>) -> String {
        let mut strings: Vec<String> = fieldset.into_iter()
            .map(|key| format!("\"{}\": {}", key, self.field_schema.get(&key).unwrap().repr()))
            .collect();
        strings.sort();
        format!("Record({{{}}})", strings.join(", "))
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RustFieldSet {
    pub content: HashSet<String>
}
impl RustFieldSet {
    fn new(content: HashSet<String>) -> RustFieldSet {
        RustFieldSet {
            content: content
        }
    }
    pub fn to_vec(&self) -> Vec<String> {
        let mut strings: Vec<String> = self.content.clone().into_iter().collect();
        strings.sort();
        strings
    }
    pub fn repr(&self) -> String {
        let strings: Vec<String> = self.to_vec().iter()
            .map(|s| format!("'{}'",s))
            .collect();
        format!("FieldSet({{{}}})", strings.join(", "))
    }
}

impl Hash for RustFieldSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.repr().hash(state)
    }
}

use super::atomic::*;
use super::unions::reduce;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_record() {
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        // Test constructor from RustRecord
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom.clone());
        map.insert("apple".to_owned(), str_atom.clone());
        let rr = RustRecord::new(map);
        assert_eq!(rr.field_comb_counter.keys().len(), 1);
        assert_eq!(rr.field_counter.keys().len(), 2);
        // Test constructor from RustJsonSchema
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom.clone());
        map.insert("apple".to_owned(), str_atom.clone());
        let rr = RustRecord::new(map);
        assert_eq!(rr.repr_normal(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})");
    }
    #[test]
    fn test_record_repr_normal() {
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom.clone());
        map.insert("apple".to_owned(), str_atom.clone());
        let rr = RustRecord::new(map.clone());
        assert_eq!(rr.repr_normal(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})");
        let r_schema = RustJsonSchema::Record(rr.clone());
        let complex_r_schema = r_schema.clone().merge(r_schema.clone());
        match complex_r_schema.clone() {
            RustJsonSchema::Record(r) => {
                assert_eq!(r.repr_normal(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})");
            },
            _ => {
                panic!();
            }
        }
        map.insert("pie".to_owned(), str_atom.clone());
        let rr_complex = RustRecord::new(map.clone());
        let rc_schema = RustJsonSchema::Record(rr_complex.clone());
        let complex_rc_schema = rc_schema.merge(complex_r_schema.clone());
        match complex_rc_schema.clone() {
            RustJsonSchema::Record(r) => {
                assert_eq!(r.repr_normal(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non()), \"pie\": Atomic(Str())})");
            },
            _ => {
                panic!();
            }
        }
    }
    #[test]
    fn test_record_repr_co_occurrence() {
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom.clone());
        map.insert("apple".to_owned(), str_atom.clone());
        let rr = RustRecord::new(map.clone());
        assert_eq!(rr.repr_co_occurence(), "Union({Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})})");
        let r_schema = RustJsonSchema::Record(rr.clone());
        let complex_r_schema = r_schema.clone().merge(r_schema.clone());
        match complex_r_schema.clone() {
            RustJsonSchema::Record(r) => {
                assert_eq!(r.repr_co_occurence(), "Union({Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})})");
            },
            _ => {
                panic!();
            }
        }
        map.insert("pie".to_owned(), str_atom.clone());
        let rr_complex = RustRecord::new(map.clone());
        let rc_schema = RustJsonSchema::Record(rr_complex.clone());
        let complex_rc_schema = rc_schema.merge(complex_r_schema.clone());
        match complex_rc_schema.clone() {
            RustJsonSchema::Record(r) => {
                assert_eq!(r.repr_co_occurence(), "Union({Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non()), \"pie\": Atomic(Str())}), Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})})");
            },
            _ => {
                panic!();
            }
        }
    }
    #[test]
    fn test_record_repr_uniform() {
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), str_atom.clone());
        map.insert("apple".to_owned(), str_atom.clone());
        map.insert("can".to_owned(), str_atom.clone());
        let rr = RustRecord::new(map.clone());
        let values_vec: Vec<RustJsonSchema> = rr.field_schema.values().cloned().collect();
        assert_eq!(rr.repr_uniform(reduce(values_vec)), "UniformRecord(FieldSet({'apple', 'banana', 'can'}), Atomic(Str()))");
    }
    #[test]
    fn test_record_repr() {
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        // Test simple case: 
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom.clone());
        map.insert("apple".to_owned(), str_atom.clone());
        let r_schema = RustJsonSchema::Record(RustRecord::new(map.clone()));
        match r_schema.clone() {
            RustJsonSchema::Record(r) => {
                assert_eq!(r.repr(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())})");
            },
            _ => {
                panic!();
            }
        }
        // Test union case: 
        let mut map = HashMap::new();
        map.insert("can".to_owned(), non_atom.clone());
        let p_schema = RustJsonSchema::Record(RustRecord::new(map.clone()));
        match r_schema.clone().merge(p_schema).clone() {
            RustJsonSchema::Record(r) => {
                assert_eq!(r.repr(), "Union({Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non())}), Record({\"can\": Atomic(Non())})})");
            },
            _ => {
                panic!();
            }
        }
        // Test normal case: 
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), non_atom.clone());
        map.insert("apple".to_owned(), str_atom.clone());
        map.insert("can".to_owned(), non_atom.clone());
        let p_schema = RustJsonSchema::Record(RustRecord::new(map.clone()));
        match r_schema.merge(p_schema).clone() {
            RustJsonSchema::Record(r) => {
                assert_eq!(r.repr(), "Record({\"apple\": Atomic(Str()), \"banana\": Atomic(Non()), \"can\": Atomic(Non())})");
            },
            _ => {
                panic!();
            }
        }
        // Test uniform case:
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let mut map = HashMap::new();
        map.insert("banana".to_owned(), str_atom.clone());
        map.insert("apple".to_owned(), str_atom.clone());
        map.insert("can".to_owned(), str_atom.clone());
        let s_schema = RustJsonSchema::Record(RustRecord::new(map.clone()));
        match s_schema {
            RustJsonSchema::Record(r) => {
                assert_eq!(r.repr(), "UniformRecord(FieldSet({'apple', 'banana', 'can'}), Atomic(Str()))");
            },
            _ => {
                panic!();
            }
        }
    }
}