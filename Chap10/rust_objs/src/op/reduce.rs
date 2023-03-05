use crate::schema::top::RustJsonSchema;
////////////// Main Function ///////////////////////
pub fn reduce(batch: Vec<RustJsonSchema>) -> RustJsonSchema {
    let result = batch.iter().fold(batch[0].clone(), |x, y| x.merge(y.clone()));
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
        let batch = vec![str_atom, bool_atom, non_atom];
        let result = reduce(batch);
        assert_eq!(result.repr(), "Union({Atomic(Bool()), Atomic(Non()), Atomic(Str())})");
    }
}