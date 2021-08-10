import pytest
from adapters import parameter_info

@pytest.mark.parametrize('input', ["garbage",12, 37.5])
def test_parameter_info_not_valid(input):
    assert parameter_info(input) is None

@pytest.mark.parametrize(('category', 'expected'), [
    ("symbols", [{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}, {"Parameter":"frequency","Property":"ω"}]),
    ("groups", [{"Parameter":"frequency","Property":"System parameters"},{"Parameter":"offset","Property":"System parameters"},{"Parameter":"r","Property":"System parameters"},{"Parameter":"phi","Property":"Initial impact"},{"Parameter":"v","Property":"Initial impact"},{"Parameter":"maxPeriods","Property":"Control parameters"},{"Parameter":"numIterations","Property":"Control parameters"},{"Parameter":"numPoints","Property":"Control parameters"}]),
    ("garbage", None)
])
def test_parameter_info_valid(category, expected):
    info = parameter_info(category)
    if expected is None:
        assert info == expected
    else:
        assert "Properties" in info

        assert info["Properties"] == expected