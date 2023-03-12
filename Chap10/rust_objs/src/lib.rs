// Goal: 
// Make the complex object computation fast. 
// - The complex object computation should be in the Rust domain (implemented with pure Rust objects). 
// - A `reduce` method operate on the Rust domain (input: json str list, output: Object str)
// - The Object str can be transformed to Python object in Python domain. 
// - When doing `|` operation in Python domain, `or` in Rust domain should be called.  
// FEAT TODO:
// 1. [X] Build pure rust objects 
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
//    - [X] Record
//         - [X] Basic Representation 
//         - [X] UniformRecord: Uniform Representation (Merge all schemas) (UniformRecord(FieldSet({"A", "B", "C"}, UNION_SCHEMA)))
//               - [X] Build FieldSet python objects (+modify FieldSet to RustFieldSet) that Takes set of strings as input 
//               - [X] Build UniformRecord Python Objects that build RustRecords from FieldSet and the Union JsonSchema. 
//         - [X] Record(): Dynamic Representation 1 (show fields and their schemas with fields).
//         - [X] Union({Record()}): Dynamic Representation 2 (show only the field combination and their schemas as different Records):  Union({Record(xx), Record(xx)})
//         - [X] Auto decide representation by constraint. 
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
// 4. [X] Refactor into multiple files
//    - [X] Seperate RustObjs and PythonObjs
// 5. [X] Mapping of Json String to RustJsonSchema using JsonSerdson...
// 6. [X] Implement methods on Rust objects and call them from the Python Object. 
// 7. [X] UnitTest identical to the jsonschema python package.
use pyo3::prelude::*;
use pyo3::types::PyList;
use num_cpus;
mod op;
use op::infer::RustInferenceEngine;
use op::reduce::reduce;
mod schema;
use schema::top::RustJsonSchema;
use schema::atomic::atomic::{Non, Str, Bool, Atomic};
use schema::atomic::num::{Float, Int};
use schema::record::{Record, FieldSet, UniformRecord};
use schema::array::{Array};
use schema::unions::{Union, Optional};
use schema::unknown::Unknown;
use schema::convert::py2rust;
//////////////////// Reduce Merge of Json Schemas ///////////////////////
#[pyclass]
struct InferenceEngine {
    rust_obj: RustInferenceEngine
}
// 
#[pymethods]
impl InferenceEngine {
    #[new]
    fn new(cpu_cnt: Option<i32>) -> PyResult<Self> {
        let set_cnt = match cpu_cnt {
            Some(val) => val as usize,
            _ => num_cpus::get()
        };
        println!("Thread Count: {}", set_cnt);
        Ok(InferenceEngine { rust_obj: RustInferenceEngine::new(set_cnt)})
    }
    fn run(&self, batch: &PyList) -> String {
        let vec: Vec<&str> = (0..batch.len())
            .map(|i| batch.get_item(i).unwrap().extract::<&str>().unwrap())
            .collect();
        self.rust_obj.infer(vec)
    }
    fn reduce(&self, batch: &PyList) -> String {
        let s_vec: Vec<RustJsonSchema> = (0..batch.len())
            .map(|i| batch.get_item(i).unwrap().extract::<&PyAny>().unwrap())
            .map(|s| py2rust(s))
            .collect();
        reduce(s_vec).repr()
    }
}
#[pymodule]
fn rust_objs( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_class::<InferenceEngine>()?;
    m.add_class::<Int>()?;
    m.add_class::<Float>()?;
    m.add_class::<Str>()?;
    m.add_class::<Non>()?;
    m.add_class::<Bool>()?;
    m.add_class::<Atomic>()?;
    m.add_class::<Array>()?;
    m.add_class::<Record>()?;
    m.add_class::<FieldSet>()?;
    m.add_class::<UniformRecord>()?;
    m.add_class::<Union>()?;
    m.add_class::<Optional>()?;
    m.add_class::<Unknown>()?;
    return Ok( () );
}