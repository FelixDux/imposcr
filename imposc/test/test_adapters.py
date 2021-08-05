import pytest
from adapters import parameter_info

@pytest.mark.parametrize('input', ["garbage",12, 37.5])
def test_parameter_info_not_valid(input):
    assert parameter_info(input) is None

@pytest.mark.parametrize(('category', 'expected'), [
    ("symbols", {"Properties":[{"Parameter":"frequency","Property":"ω"},{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}]}),
    ("garbage", None)
])
def test_parameter_info_valid(category, expected):
    assert parameter_info(category) == expected