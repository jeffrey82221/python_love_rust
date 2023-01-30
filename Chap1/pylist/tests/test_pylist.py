from pylist import add_one, add_one_inplace, add_one_parallel

def test_add_one():
    assert add_one([1,2,3]) == [2,3,4]

def test_add_one_parallel():
    assert add_one_parallel([1,2,3]) == [2,3,4]

def test_add_one_inplace():
    a = [1,2,3]
    add_one_inplace(a)
    assert a == [2,3,4]
