from fastapi.testclient import TestClient
import pytest

from main import app

client = TestClient(app)

def get_response_for_test(path):
    response = client.get(path)
    
    return response.json(), response.status_code

def post_response_for_test(path, input_json):
    response = client.post(path, json=input_json)

    return response.json(), response.status_code

def test_read_main():
    response = client.get("/")
    assert response.status_code == 200

@pytest.mark.parametrize(('category', 'status', 'response_json'), [
    ("symbols", 200, {"Properties":[{"Parameter":"frequency","Property":"ω"},{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}]}),
    ("groups", 200, {"Properties":[{"Parameter":"frequency","Property":"System parameters"},{"Parameter":"offset","Property":"System parameters"},{"Parameter":"r","Property":"System parameters"},{"Parameter":"phi","Property":"Initial impact"},{"Parameter":"v","Property":"Initial impact"},{"Parameter":"maxPeriods","Property":"Control parameters"},{"Parameter":"numIterations","Property":"Control parameters"},{"Parameter":"numPoints","Property":"Control parameters"}]}),
])
def test_read_parameter_info(category, status, response_json):

    json, actual_status = get_response_for_test(f"/api/parameter-info/{category}")

    assert actual_status == status
    
    assert "Properties" in json

    body = json["Properties"]
        
    for element in response_json["Properties"]:
        assert element in body

    for element in body:
        assert element in response_json["Properties"]

@pytest.mark.parametrize(('category', 'status', 'response_json'), [("garbage", 404, {"detail": "Parameter info category not found"})
])
def test_read_parameter_info_bad_path(category, status, response_json):
    json, actual_status = get_response_for_test(f"/api/parameter-info/{category}")

    assert actual_status == status
    assert json == response_json

def test_get_impact_iteration():
    input_json = {"frequency": 2.0,
    "offset": 0.0,
    "r": 0.8,
    "max_periods": 100,
    "phi": 0.0,
    "v": 0.0,
    "num_iterations": 2}
    json, actual_status = post_response_for_test(f"/api/iteration/data", input_json)
    assert actual_status == 200, f"{json}"
    assert json