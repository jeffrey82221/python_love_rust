use pyo3::prelude::*;
use serde_json::Value;
use serde_json::*;


fn check_type(val: &dyn std::any::Any) -> &'static str {
    if val.is::<i32>() {
        "i32"
    } else if val.is::<f32>() {
        "f32"
    } else if val.is::<&str>() {
        "&str"
    } else if val.is::<String>() {
        "String"
    } else if val.downcast_ref::<Value>().is_some() {
        match val.downcast_ref::<Value>().unwrap() {
            Value::Array(_) => "Value::Array",
            Value::Number(_) => "Value::Number",
            Value::String(_) => "Value::String",
            Value::Bool(_) => "Value::Bool",
            Value::Null => "Value::Null",
            Value::Object(_) => "Value::Object",
            _ => "Value",
        }
    } else {
        "Other type"
    }
}

#[pyfunction]
pub fn parse() -> PyResult<()> {
    let x = 42;
    println!("x is a {}", check_type(&x));

    let y = 42.0 as f32;
    println!("y is a {}", check_type(&y));

    let z = "hello";
    println!("z is a {}", check_type(&z));

    let w = "hello".to_string();
    println!("w is a {}", check_type(&w));

    let v = Value::Array(vec![Value::from(1), Value::from(2), Value::from(3)]);
    println!("v is a {}", check_type(&v));

    let u = Value::from(1);
    println!("u is a {}", check_type(&u));

    let l = Value::from("Apple");
    println!("l is a {}", check_type(&l));

    let m = Value::Bool(true);
    println!("m is a {}", check_type(&m));

    let n = Value::Null;
    println!("n is a {}", check_type(&n));

    let o = Value::Object(
        vec![
            (String::from("key1"), Value::String(String::from("value1"))),
            (String::from("key2"), Value::Number(Number::from(42)))
        ].into_iter().collect()
    );
    println!("o is a {}", check_type(&o));
    Ok(())
}
#[pymodule]
fn print_type_of_rust_obj( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( parse, m )? )?;
    return Ok( () );
}