from fastapi import FastAPI, HTTPException
from adapters import iterate_impacts, parameter_info, get_app_info, iterate, IterationOutputs
from fastapi.staticfiles import StaticFiles
from fastapi.responses import RedirectResponse
from pydantic import BaseModel
from logging import warn
from typing import Optional
from imposclib.imposclib import IterationInputs

app = FastAPI(**(get_app_info())) # TODO: configure info and info endpoint from file

app.mount("/static", StaticFiles(directory="./static"), name="static")

def respond_with_error(status_code: int, detail: str) -> None:
    warn(f"Raising HTTPException(status_code={status_code}, detail={detail})")
    raise HTTPException(status_code=status_code, detail=detail)

@app.get("/")
async def read_main():
    return RedirectResponse("/static/index.html")

@app.get("/api/parameter-info/{category}")
async def read_parameter_info(category):
    result = parameter_info(category)

    if result is None:
        respond_with_error(status_code=404, detail="Parameter info category not found")
    else:
        return result

class IterationPostData(BaseModel):
    frequency: float = 2.8
    offset: float = 0.0
    r: float = 0.8
    max_periods: int = 100
    phi: float = 0.0
    v: float = 0.0
    num_iterations: int = 10

    def __call__(self) -> IterationInputs:
        return IterationInputs(
            frequency = self.frequency,
            offset = self.offset,
            r = self.r,
            max_periods = self.max_periods,
            phi = self.phi,
            v = self.v,
            num_iterations = self.num_iterations)
        
@app.post("/api/iteration/data")
async def read_iteration_data(data: IterationPostData):
    if data is None:
        respond_with_error(status_code=400, detail="Form inputs not found")

    result = iterate_impacts(data())

    if result is None:
        respond_with_error(status_code=404, detail="Parameter info category not found")
    else:
        return [impact for impact in result]