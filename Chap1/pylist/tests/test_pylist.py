from pylist import add_one, add_one_inplace

def test_add_one():
    assert add_one([1,2,3]) == [2,3,4]


def test_add_one_inplace():
    a = [1,2,3]
    add_one_inplace(a)
    assert a == [2,3,4]