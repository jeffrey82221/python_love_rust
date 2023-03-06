use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use num_cpus;
use serde_json::Value;
use std::collections::HashMap;
use crate::schema::top::RustJsonSchema;
use crate::schema::atomic::{RustAtomic, RustNon, RustBool, RustStr};
use crate::schema::num::{RustNum, RustInt, RustFloat};
use crate::schema::array::RustArray;
use crate::schema::record::RustRecord;
use crate::schema::unknown::RustUnknown;
use super::reduce::reduce;
pub struct RustInferenceEngine {
    pool: ThreadPool
}
impl RustInferenceEngine {
    pub fn new() -> RustInferenceEngine {
        let pool = ThreadPoolBuilder::new()
            .num_threads(num_cpus::get() * 2)
            .build()
            .unwrap();
        RustInferenceEngine {pool: pool}
    }
    pub fn infer(&self, batch: Vec<&str>) -> String {
        let first_schema = RustJsonSchema::Unknown(RustUnknown::new());
        let reduced = self.pool.install(|| {
            batch
                .into_par_iter()
                .map(|json_str| to_schema(serde_json::from_str(json_str).unwrap()))
                .reduce(|| first_schema.clone(), |x, y| x.merge(y.clone()))
        });
        reduced.repr()
    }
}


fn to_schema(json_value: Value) -> RustJsonSchema {
    match json_value {
        Value::Null => {
            RustJsonSchema::Atomic(RustAtomic::Non(RustNon {}))
        },
        Value::Bool(_) => {
            RustJsonSchema::Atomic(RustAtomic::Bool(RustBool {}))
        },
        Value::Number(n) => {
            if n.is_i64() {
                RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Int(RustInt{})))    
            } else {
                RustJsonSchema::Atomic(RustAtomic::Num(RustNum::Float(RustFloat{})))    
            }  
        },
        Value::String(_) => {
            RustJsonSchema::Atomic(RustAtomic::Str(RustStr {}))
        },
        Value::Array(a) => {
            let vec_schema: Vec<RustJsonSchema> = a.iter()
            .map(|value| to_schema(value.clone()))
            .collect();
            RustJsonSchema::Array(RustArray { content: Box::new(reduce(vec_schema)) })
        }
        Value::Object(o) => {
            let mut mapped_schema = HashMap::new();
            for (k, v) in o.iter() {
                mapped_schema.insert(k.clone(), to_schema(v.clone()));
            }
            RustJsonSchema::Record(RustRecord::new(mapped_schema))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::op::infer::to_schema;
    use crate::op::infer::RustInferenceEngine;
    use serde_json::Value;
    use serde_json;
    #[test]
    fn test_infer() {
        let inferer = RustInferenceEngine::new();
        let jsons = vec!["1", "1.0"];
        let result = inferer.infer(jsons);
        assert_eq!(result, "Union({Atomic(Float()), Atomic(Int())})");
        let jsons = vec!["1"];
        let result = inferer.infer(jsons);
        assert_eq!(result, "Atomic(Int())");
        let jsons = vec![];
        let result = inferer.infer(jsons);
        assert_eq!(result, "Unknown()");
    }
    #[test]
    fn test_to_schema() {
        let json_str = "null";
        let ans = to_schema(serde_json::from_str(json_str).unwrap());
        assert_eq!(ans.repr(), "Atomic(Non())");
        let json_str = "true";
        let ans = to_schema(serde_json::from_str(json_str).unwrap());
        assert_eq!(ans.repr(), "Atomic(Bool())");
        let json_str = "1";
        let ans = to_schema(serde_json::from_str(json_str).unwrap());
        assert_eq!(ans.repr(), "Atomic(Int())");
        let json_str = "1.0";
        let ans = to_schema(serde_json::from_str(json_str).unwrap());
        assert_eq!(ans.repr(), "Atomic(Float())");
        let json_str = "\"apple\"";
        let ans = to_schema(serde_json::from_str(json_str).unwrap());
        assert_eq!(ans.repr(), "Atomic(Str())");
        let json_str = "[1, 2, 3.0]";
        let ans = to_schema(serde_json::from_str(json_str).unwrap());
        assert_eq!(ans.repr(), "Array(Union({Atomic(Float()), Atomic(Int())}))");
        let json_str = "{\"apple\": null}";
        let ans = to_schema(serde_json::from_str(json_str).unwrap());
        assert_eq!(ans.repr(), "Record({\"apple\": Atomic(Non())})");
    }
}