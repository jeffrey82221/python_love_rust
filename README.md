# python_love_rust
A playground for building rust binding for python

## Setup 

### Compile the string_sum package
```
cd string_sum 
source .env/bin/activate
pip install --upgrade pip
pip install maturin
maturin develop
```
### Use the package

```python
import string_sum
string_sum.sum_as_string(5, 20)
```

## TODO:

- [ ] A multi-thread example and compare the speed to the python sequantial version. 

