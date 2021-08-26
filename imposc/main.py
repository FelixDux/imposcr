from fastapi import FastAPI, HTTPException
from adapters import parameter_info
from fastapi.staticfiles import StaticFiles
from fastapi.responses import RedirectResponse

app = FastAPI(title="Impact Oscillator", description="Analysis and simulation of a simple vibro-impact model developed in Rust, with a Python wrapper - principally as a learning exercise") # TODO: configure info and info endpoint from file

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