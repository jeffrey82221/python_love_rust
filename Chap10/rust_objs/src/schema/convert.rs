use pyo3::prelude::*;
use super::top::RustJsonSchema;
use super::atomic::atomic::Atomic;
use super::array::Array;
use super::record::{Record, UniformRecord};
use super::unions::{Union, Optional};
use super::unknown::Unknown;
pub fn py2rust(value: &PyAny) -> RustJsonSchema {
    /*
    Convert PyAny to its Rust Counterpart: 
    */
    let rust_schema = match (
        value.extract::<Atomic>(), 
        value.extract::<Array>(), 
        value.extract::<Record>(), 
        value.extract::<UniformRecord>(), 
        value.extract::<Union>(), 
        value.extract::<Optional>(), 
        value.extract::<Unknown>()
    ) {
        (Ok(atom), _, _, _, _, _, _) => RustJsonSchema::Atomic(atom.rust_obj),
        (_, Ok(arr), _, _, _, _, _) => RustJsonSchema::Array(arr.rust_obj),
        (_, _, Ok(rec), _, _, _, _) => RustJsonSchema::Record(rec.rust_obj),
        (_, _, _, Ok(rec), _, _, _) => RustJsonSchema::Record(rec.rust_obj),
        (_, _, _, _, Ok(uni), _, _) => RustJsonSchema::Union(uni.rust_obj),
        (_, _, _, _, _, Ok(uni), _) => RustJsonSchema::Union(uni.rust_obj),
        (_, _, _, _, _, _, Ok(unk)) => RustJsonSchema::Unknown(unk.rust_obj),
        _ => panic!("Expect Atomic, Array, Record, UniformRecord, Union, Optional, or Unknown")
    };
    rust_schema
}