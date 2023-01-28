# Intro

creating the simple python package with Rust function 

# Development Step:

1) create string_sum folder

```bash
mkdir string_sum
cd string_sum
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

5) Execute the `string_sum` module on Python:

```python
import string_sum
string_sum.sum_as_string(5, 20)
>> '25'
```

# TODO:

- [ ] add rust unit test
- [ ] add python functional test
