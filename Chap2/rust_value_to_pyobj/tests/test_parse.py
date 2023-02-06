from rust_value_to_pyobj import parse 

def test_parse():
    assert parse() == {1: {'test11': 'Foo', 'test12': 123}, 2: {'test21': 'Bar', 'test22': 123.45}}