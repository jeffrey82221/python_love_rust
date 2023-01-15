# python_love_rust
A playground for building rust binding for python

# Setup 

1) Install maturin 

`pip install maturin`

2) Goto First Example Folder 

`cd first_example`

3) Initialize maturin 

`maturin init`

```
✔ 🤷 Which kind of bindings to use?
  📖 Documentation: https://maturin.rs/bindings.html · pyo3
  ✨ Done! Initialized project /Users/jeffreylin/codespace/python_love_rust
```

4) Build develop

`maturin develop`

5) Run the Rust Program from Python

```
import string_sum 
string_sum.sum_as_string(1,3)
> '4'
```

# Tutorials

ref: https://towardsdatascience.com/learning-rust-by-converting-python-to-rust-259e735591c6


## Python Func vs Rust Func:

## Python Class vs Rust Struct: 

## Python Inheritance vs Rust Trait Implementation

## Python Obj Type Checking vs Rust Type Checking