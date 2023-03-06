use pyo3::prelude::*;
////////////////// PyObjs ///////////////////
#[derive(Clone, Copy)]
#[pyclass]
pub struct Float {
    pub rust_obj: RustFloat,
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

#[derive(Clone, Copy)]
#[pyclass]
pub struct Int {
    pub rust_obj: RustInt,
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

////////////////// RustObjs ///////////////////
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum RustNum {
    Int(RustInt),
    Float(RustFloat)
}
impl RustNum {
    pub fn repr(&self) -> String {
        match self {
            RustNum::Int(int_val) => int_val.repr(),
            RustNum::Float(float_val) => float_val.repr(),
        }
    }
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RustFloat {}
impl RustFloat {
    fn new() -> RustFloat {
        RustFloat {}
    }
    pub fn repr(&self) -> String {
        format!("Float()")
    }
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RustInt {}
impl RustInt {
    fn new() -> RustInt {
        RustInt {}
    }
    pub fn repr(&self) -> String {
        format!("Int()")
    }
}


#[cfg(test)]
mod num_tests {
    use super::*;
    #[test]
    fn test_repr() {
        let int_num = RustNum::Int(RustInt::new());
        assert_eq!(int_num.repr(), "Int()");
        let float_num = RustNum::Float(RustFloat::new());
        assert_eq!(float_num.repr(), "Float()");
    }
}