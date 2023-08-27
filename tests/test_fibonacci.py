import pytest
from python.fibonacci import fibonacci as py_fibonacci
from fibonacci import rust_fibonacci

fibonacci_sequence = [
    (0, 0),
    (1, 1),
    (9, 34),
    (10, 55),
    (12, 144),
    (14, 377),
]

@pytest.mark.parametrize("n, expected", fibonacci_sequence)    
def test_py_fibonacci(n, expected):
    assert py_fibonacci(n) == expected

@pytest.mark.parametrize("n, expected", fibonacci_sequence)    
def test_rust_fibonacci(n, expected):
    assert rust_fibonacci(n) == expected
