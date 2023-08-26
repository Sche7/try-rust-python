import pytest
from src.fibonacci import fibonacci


@pytest.mark.parametrize("n, expected",[
    (0, 0),
    (1, 1),
    (9, 34),
    (10, 55),
    (12, 144),
    (14, 377),
])    
def test_fibonacci(n, expected):
    assert fibonacci(n) == expected
