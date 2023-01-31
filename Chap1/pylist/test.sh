pip install -e .
python -m pytest
echo 'wo parallel'
python -m timeit 'from pylist import self_powering; c = self_powering(list(range(int(10e5), int(10e6))))'
echo 'w parallel'
python -m timeit 'from pylist import self_powering_parallel; c = self_powering_parallel(list(range(int(10e5),int(10e6))))'