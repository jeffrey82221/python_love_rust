use pyo3::prelude::{pyclass, pymethods};

#[pyclass]
pub struct LogicUnit {
    #[pyo3(get, set)]
    pub a: i32,
    #[pyo3(get, set)]
    pub b: i32
}

#[pymethods]
impl LogicUnit {

    #[new]
    fn new(a: i32, b: i32) -> Self {
        return LogicUnit {a, b}
    }

    fn sum(&self) -> i32 {
        return self.a + self.b
    }

    fn mul(&self) -> i32 {
        return self.a * self.b
    }

    fn copy(&self) -> Self {
        return LogicUnit {a: self.a, b: self.b}
    }
}

#[test]
fn test_sum() {
    let l: LogicUnit = LogicUnit {a: 1, b: 1};
    assert_eq!(l.sum(), 2);
    let l: LogicUnit = LogicUnit {a: 1, b: -1};
    assert_eq!(l.sum(), 0);
}

#[test]
fn test_mul() {
    let l: LogicUnit = LogicUnit {a: 1, b: 1};
    assert_eq!(l.mul(), 1);
    let l: LogicUnit = LogicUnit {a: 2, b: 3};
    assert_eq!(l.mul(), 6);
}