use std::collections::HashMap;

use pyo3::prelude::*;
use serde_json::Value;

fn value_to_object( val: &Value, py: Python<'_> ) -> PyObject {
    match val {
        Value::Null => py.None(),
        Value::Bool( x ) => x.to_object( py ),
        Value::Number( x ) => {
            let oi64 = x.as_i64().map( |i| i.to_object( py ) );
            let ou64 = x.as_u64().map( |i| i.to_object( py ) );
            let of64 = x.as_f64().map( |i| i.to_object( py ) );
            oi64.or( ou64 ).or( of64 ).expect( "number too large" )
        },
        Value::String( x ) => x.to_object( py ),
        Value::Array( x ) => {
            let inner: Vec<_> = x.iter().map(|x| value_to_object(x, py)).collect();
            inner.to_object( py )
        },
        Value::Object( x ) => {
            let inner: HashMap<_, _> =
                x.iter()
                    .map( |( k, v )| ( k, value_to_object( v, py ) ) ).collect();
            inner.to_object( py )
        },
    }
}

#[repr(transparent)]
#[derive( Clone, Debug )]
struct ParsedValue( Value );

impl ToPyObject for ParsedValue {
    fn to_object( &self, py: Python<'_> ) -> PyObject {
        value_to_object( &self.0, py )
    }
}

#[pyfunction]
pub fn parse() -> PyResult<PyObject> {
    let mapping: HashMap<i64, HashMap<String, ParsedValue>> = HashMap::from( [
        ( 1, HashMap::from( [
            ( "test11".to_string(), ParsedValue( "Foo".into() ) ),
            ( "test12".to_string(), ParsedValue( 123.into() ) ),
        ] ) ),
        ( 2, HashMap::from( [
            ( "test21".to_string(), ParsedValue( "Bar".into() ) ),
            ( "test22".to_string(), ParsedValue( 123.45.into() ) ),
        ] ) ),
    ] );

    Ok( pyo3::Python::with_gil( |py| {
        mapping.to_object( py )
    } ) )
}

#[pymodule]
fn rust_value_to_pyobj( _py: Python, m: &PyModule ) -> PyResult<()> {
    m.add_function( wrap_pyfunction!( parse, m )? )?;

    return Ok( () );
}