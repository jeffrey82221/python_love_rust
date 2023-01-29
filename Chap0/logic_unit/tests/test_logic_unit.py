from logic_unit import LogicUnit

def test_sum():
    l = LogicUnit(1, 1)
    assert l.sum() == 2

def test_mul():
    l = LogicUnit(1, 1)
    assert l.mul() == 1

def test_copy():
    l = LogicUnit(1, 1)
    assert l.copy() != l
    l2 = l 
    assert l2 == l