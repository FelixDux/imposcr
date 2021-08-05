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