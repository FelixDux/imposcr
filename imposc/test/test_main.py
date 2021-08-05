from fastapi.testclient import TestClient

from main import app

client = TestClient(app)

def test_read_main():
    response = client.get("/")
    assert response.status_code == 200
    assert response.json() == {"msg": "Watch this space ..."}

def test_read_inexistent_item():
    response = client.get("/api/parameter-info/houses")
    assert response.status_code == 404
    assert response.json() == {"detail": "Not Found"}

def test_read_parameter_symbols():
    response = client.get("/api/parameter-info/symbols")
    assert response.status_code == 200
    assert response.json() == {"Properties":[{"Parameter":"frequency","Property":"ω"},{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}]}

def test_read_parameter_groups():
    response = client.get("/api/parameter-info/groups")
    assert response.status_code == 200
    assert response.json() == {"Properties":[{"Parameter":"frequency","Property":"System parameters"},{"Parameter":"offset","Property":"System parameters"},{"Parameter":"r","Property":"System parameters"},{"Parameter":"phi","Property":"Initial impact"},{"Parameter":"v","Property":"Initial impact"},{"Parameter":"maxPeriods","Property":"Control parameters"},{"Parameter":"numIterations","Property":"Control parameters"},{"Parameter":"numPoints","Property":"Control parameters"}]}