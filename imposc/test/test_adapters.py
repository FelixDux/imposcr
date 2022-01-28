import pytest
from adapters import parameter_info, validate_iter_inputs, IterationInputs

@pytest.mark.parametrize("inputs", [
    {
        'frequency': 1.2,
        'offset': -0.7,
        'r': 0.1,
        'max_periods': 58,
        'phi': 0.1,
        'v': 1.1,
        'num_iterations': 35},
])
def test_validate_iter_inputs(inputs):
    iteration_inputs = IterationInputs(**inputs)
    result = validate_iter_inputs(iteration_inputs)

    for key, value in inputs.items():
        assert hasattr(result, key)
        assert getattr(result, key)() == value
    

@pytest.mark.parametrize('input', ["garbage",12, 37.5])
def test_parameter_info_not_valid(input):
    assert parameter_info(input) is None

@pytest.mark.parametrize(('category', 'expected'), [
    ("symbols", [{"Parameter":"offset","Property":"σ"},
    
    {"Parameter":"phi","Property":"φ"}, 
    {"Parameter":"frequency","Property":"ω"}]),
    ("groups", [{"Parameter":"frequency","Property":"System parameters"},
    {"Parameter":"offset","Property":"System parameters"},
    {"Parameter":"r","Property":"System parameters"},
    {"Parameter":"phi","Property":"Initial impact"},
    {"Parameter":"v","Property":"Initial impact"},
    {"Parameter":"max_periods","Property":"Control parameters"},
    {"Parameter":"num_iterations","Property":"Control parameters"},
    {"Parameter":"num_points","Property":"Control parameters"}]),
    ("garbage", None)
])
def test_parameter_info_valid(category, expected):
    info = parameter_info(category)
    if expected is None:
        assert info == expected
    else:
        assert "Properties" in info

        properties = info["Properties"]
        
        for element in expected:
            assert element in properties

        for element in properties:
            assert element in expected