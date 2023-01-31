pip install -e .
python -m pytest
python -m timeit 'from pylist import add_one; c = add_one([1]*int(10e6))'
python -m timeit 'from pylist import add_one_parallel; c = add_one_parallel([1]*int(10e6))'