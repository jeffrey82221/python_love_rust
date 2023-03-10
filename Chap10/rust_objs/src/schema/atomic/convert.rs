use pyo3::prelude::*;
use super::atomic::{RustAtomic, Str, Non, Bool};
use super::num::{RustNum, Int, Float};
pub fn py2rust(obj: &PyAny) -> RustAtomic {
    /*
    Convert PyAny to its Rust Counterpart: 
    */
    let rust_atomic = match (
        obj.extract::<Int>(), 
        obj.extract::<Float>(), 
        obj.extract::<Str>(), 
        obj.extract::<Non>(), 
        obj.extract::<Bool>()
    ) {
        (Ok(x), _, _, _, _) => RustAtomic::Num(RustNum::Int(x.rust_obj)),
        (_, Ok(x), _, _, _) => RustAtomic::Num(RustNum::Float(x.rust_obj)),
        (_, _, Ok(x), _, _) => RustAtomic::Str(x.rust_obj),
        (_, _, _, Ok(x), _) => RustAtomic::Non(x.rust_obj),
        (_, _, _, _, Ok(x)) => RustAtomic::Bool(x.rust_obj),
        _ => panic!("Expect an Int, Float, Str, Bool, or Non")
    };
    rust_atomic
}