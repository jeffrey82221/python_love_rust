# Intro

creating the simple python package with Rust function 

# Development Step:

1) create project folder

```bash
mkdir logic_unit
cd logic_unit
```

2) Install maturin (-> A rust + python development sdk)

```bash
pip install maturin
```

3) Initialize the Project 
```bash
maturin init
-> select pyo3
```

4) Install the Python package:

```bash
pip install -e .
``` 

```

5) Testing the Rust units:

```bash
cargo test
```

6) Testing the Python Interface:

```bash
python -m pytest
```