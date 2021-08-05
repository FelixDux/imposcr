from fastapi import FastAPI
from adapters import parameter_info

app = FastAPI()


@app.get("/")
async def read_main():
    return {"msg": "Watch this space ..."}

@app.get("/api/parameter-info/symbols")
async def read_parameter_symbols():
    return parameter_info("symbols")

@app.get("/api/parameter-info/groups")
async def read_parameter_symbols():
    return parameter_info("groups")