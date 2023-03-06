use crate::schema::top::RustJsonSchema;
use crate::schema::unknown::RustUnknown;
////////////// Main Function ///////////////////////
pub fn reduce(batch: Vec<RustJsonSchema>) -> RustJsonSchema {
    let result = batch.iter().fold(RustJsonSchema::Unknown(RustUnknown::new()), |x, y| x.merge(y.clone()));
    result
}

#[cfg(test)]
mod tests {
    use crate::op::reduce::reduce;
    use crate::schema::atomic::RustNon;
    use crate::schema::atomic::RustBool;
    use crate::schema::atomic::RustStr;
    use crate::schema::atomic::RustAtomic;
    use crate::schema::top::RustJsonSchema;
    #[test]
    fn test_reduce() {
        let str_atom = RustJsonSchema::Atomic(RustAtomic::Str(RustStr{}));
        let bool_atom = RustJsonSchema::Atomic(RustAtomic::Bool(RustBool{}));
        let non_atom = RustJsonSchema::Atomic(RustAtomic::Non(RustNon{}));
        let batch = vec![str_atom.clone(), bool_atom, non_atom];
        assert_eq!(reduce(batch).repr(), "Union({Atomic(Bool()), Atomic(Non()), Atomic(Str())})");
        let batch = vec![str_atom];
        assert_eq!(reduce(batch).repr(), "Atomic(Str())");
        let batch = vec![];
        assert_eq!(reduce(batch).repr(), "Unknown()");
    }
}