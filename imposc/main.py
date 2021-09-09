from fastapi import FastAPI, HTTPException
from adapters import parameter_info, get_app_info
from fastapi.staticfiles import StaticFiles
from fastapi.responses import RedirectResponse
from pydantic import BaseModel
from typing import Optional

app = FastAPI(**(get_app_info())) # TODO: configure info and info endpoint from file

app.mount("/static", StaticFiles(directory="./static"), name="static")

@app.get("/")
async def read_main():
    return RedirectResponse("/static/index.html")

@app.get("/api/parameter-info/{category}")
async def read_parameter_info(category):
    result = parameter_info(category)

    if result is None:
        raise HTTPException(status_code=404, detail="Parameter info category not found")
    else:
        return result

class IterationPostData(BaseModel):
    frequency: float
    offset: float
    r: float
    max_periods: int
    phi: float
    v: float
    num_iterations: int
        
@app.post("/api/iteration/data")
async def read_iteration_data(data: IterationPostData):
    if data is None:
        raise HTTPException(status_code=400, detail="Form inputs not found")

    result = dict([('a','b')])

    if result is None:
        raise HTTPException(status_code=404, detail="Parameter info category not found")
    else:
        return result