# python_love_rust
A playground for building rust binding for python

## Setup 

## Setup environment:

```
python3 -m venv .env
source .env/bin/activate

```
### Compile the string_sum package
```
cd string_sum 
source .env/bin/activate
pip install maturin
maturin develop
```
### Use the package

```python
import string_sum
string_sum.sum_as_string(5, 20)
```

## TODO:

- [ ] A multi-thread example and compare the speed to a python sequantial version. 

