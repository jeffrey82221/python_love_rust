from pydict import add_one

def test_add_one():
    assert add_one({'input': 1}) == {'input': 1, 'output': 2}
