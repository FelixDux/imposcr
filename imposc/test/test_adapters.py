import pytest
from adapters import parameter_info

def test_parameter_info_valid():
    assert parameter_info("symbols") == {"Properties":[{"Parameter":"frequency","Property":"ω"},{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}]}

def test_parameter_info_not_valid():
    assert parameter_info("garbage") is None