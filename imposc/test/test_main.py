from fastapi.testclient import TestClient
import pytest

from main import app

client = TestClient(app)

def test_read_main():
    response = client.get("/")
    assert response.status_code == 200
    assert response.json() == {"msg": "Watch this space ..."}

@pytest.mark.parametrize(('category', 'status', 'response_json'), [
    ("symbols", 200, {"Properties":[{"Parameter":"frequency","Property":"ω"},{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}]}),
    ("groups", 200, {"Properties":[{"Parameter":"frequency","Property":"System parameters"},{"Parameter":"offset","Property":"System parameters"},{"Parameter":"r","Property":"System parameters"},{"Parameter":"phi","Property":"Initial impact"},{"Parameter":"v","Property":"Initial impact"},{"Parameter":"maxPeriods","Property":"Control parameters"},{"Parameter":"numIterations","Property":"Control parameters"},{"Parameter":"numPoints","Property":"Control parameters"}]}),
    ("garbage", 404, {"detail": "Parameter info category not found"})
])
def test_read_parameter_info(category, status, response_json):
    response = client.get(f"/api/parameter-info/{category}")
    assert response.status_code == status
    assert response.json() == response_json