from fastapi import FastAPI, HTTPException
from adapters import parameter_info

app = FastAPI(title="Impact Oscillator") # TODO: configure info and info endpoint from file


@app.get("/")
async def read_main():
    return {"msg": "Watch this space ..."}

@app.get("/api/parameter-info/{category}")
async def read_parameter_info(category):
    result = parameter_info(category)

    if result is None:
        raise HTTPException(status_code=404, detail="Parameter info category not found")
    else:
        return result