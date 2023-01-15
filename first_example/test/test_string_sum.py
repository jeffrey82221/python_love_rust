import python.string_sum
import string_sum

def test_python_string_sum():
    assert python.string_sum.sum_as_string(1, 2) == '3'

def test_rust_string_sum():
    assert string_sum.sum_as_string(1, 2) == '3'