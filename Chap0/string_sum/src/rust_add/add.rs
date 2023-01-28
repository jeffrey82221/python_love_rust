pub fn add(a: usize, b: usize) -> String {
    let c: String = (a + b).to_string();
    return c
}

pub fn hello_call(input_str: &str) {
    println!("Hello! {}", input_str)
}

// Unit Test: `cargo test`
#[test]
fn test_add() {
    assert_eq!(add(1, 1), '2'.to_string());
    assert_eq!(add(1, 5), '6'.to_string());
}

#[test]
fn test_hello_call() {
    hello_call("dance")
}