from pyset import convert_set_to_list


def test_convert_set_to_list():
    assert convert_set_to_list({1,3,2}) == [1,2,3]