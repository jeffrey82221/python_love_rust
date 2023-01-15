import python.operation
import operation
def test_python_class():
    assert python.operation.run_class('add', 1, 2) == 3
    assert python.operation.run_class('mul', 3, 2) == 6
    assert python.operation.run_class('sub', 3, 2) == 1
    assert python.operation.run_class('div', 3, 2) == 1.5

def test_rust_class():
    assert operation.run_class('add', 1, 2) == 3
    assert operation.run_class('mul', 3, 2) == 6
    assert operation.run_class('sub', 3, 2) == 1
    assert operation.run_class('div', 3, 2) == 1.5