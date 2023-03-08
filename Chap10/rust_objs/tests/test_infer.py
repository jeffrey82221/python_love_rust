import pytest
from rust_objs import InferenceEngine
from rust_objs import *

@pytest.fixture
def engine():
    return InferenceEngine()

@pytest.fixture
def test_data():
    return [
        (["1"], "Atomic(Int())"),
        (["1.2"], "Atomic(Float())"),
        (["[1]"], "Array(Atomic(Int()))"),
        (['{"a": 1, "b": 2.0}'], 'Record({"a": Atomic(Int()), "b": Atomic(Float())})'),
        (["1", "null"], 'Optional(Atomic(Int()))'),
        (["1", "1.2", "null"], 'Union({Atomic(Float()), Atomic(Int()), Atomic(Non())})'),
        (['{"a": 1, "b": 2.0}', '{"a": 1, "b": 5.0}'], 'Record({"a": Atomic(Int()), "b": Atomic(Float())})'),
        ([], 'Unknown()'),
        (['{"1": {"a": 5, "b": 6.0}, "2": {"a": 34, "b": 3.0 } }'], 'UniformRecord(FieldSet({"1", "2"}), Record({"a": Atomic(Int()), "b": Atomic(Float())}))'),
        (['{"1": [{"a": 5, "b": 6.0}], "2": [{"a": 34, "b": 3.0 }] }'], 'UniformRecord(FieldSet({"1", "2"}), Array(Record({"a": Atomic(Int()), "b": Atomic(Float())})))'),
        (['{"1": [{"a": 5, "b": 6.0}], "2": [] }'], 'UniformRecord(FieldSet({"1", "2"}), Array(Record({"a": Atomic(Int()), "b": Atomic(Float())})))'),
        (['{"1": [{"a": 5, "b": 6.0}], "2": [{"a": 34, "b": null }] }'], 'UniformRecord(FieldSet({"1", "2"}), Array(Record({"a": Atomic(Int()), "b": Optional(Atomic(Float()))})))'),
    ]

def test_schema_class_ok(test_data):
    for _, schema_str in test_data:
        assert eval(schema_str).__repr__() == schema_str


def test_inference_ok(engine, test_data):
    for json_list, schema_str in test_data:
        assert engine.run(json_list) == schema_str