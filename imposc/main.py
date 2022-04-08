from pathlib import Path
from dataclasses import dataclass
import io
from fastapi import FastAPI, HTTPException, Depends, Query
from adapters import iterate_impacts, parameter_info, get_app_info
from fastapi.staticfiles import StaticFiles
from fastapi.responses import RedirectResponse
from starlette.responses import StreamingResponse
from logging import warning
from imposclib.imposclib import IterationInputs
from charts import scatter_plot

app = FastAPI(**(get_app_info())) # TODO: configure info and info endpoint from file

app.mount("/static", StaticFiles(directory="./static"), name="static")

def respond_with_error(status_code: int, detail: str) -> None:
    warning(f"Raising HTTPException(status_code={status_code}, detail={detail})")
    raise HTTPException(status_code=status_code, detail=detail)

@app.get("/")
async def read_main():
    return RedirectResponse("/static/index.html")

@app.get("/favicon.ico")
async def read_favicon():
    return RedirectResponse("/static/favicon.ico")

@app.get("/spinner.html")
async def read_spinner():
    return RedirectResponse("/static/spinner.html")

@app.get("/api/parameter-info/{category}")
async def read_parameter_info(category):
    result = parameter_info(category)

    if result is None:
        respond_with_error(status_code=404, detail="Parameter info category not found")
    else:
        return result

@dataclass
class IterationQueryData:
    frequency: float = Query(2.8, title="Forcing frequency", gt=0)
    offset: float = Query(0.0, title="Obstacle offset")
    r: float = Query(0.8, title="Coefficient of restitution", ge=0, le=1)
    max_periods: int = Query(100, title="Number of periods without an impact after which the algorithm will report 'long excursions'", gt=0)
    phi: float = Query(0.5, title="Phase at initial impact")
    v: float = Query(0.0, title="Velocity at initial impact")
    num_iterations: int = Query(5000, title="Number of iterations of impact map")

    def __call__(self) -> IterationInputs:
        return IterationInputs(
            frequency = self.frequency,
            offset = self.offset,
            r = self.r,
            max_periods = self.max_periods,
            phi = self.phi,
            v = self.v,
            num_iterations = self.num_iterations)
        
@app.get("/api/iteration/data")
async def read_iteration_data(data: IterationQueryData=Depends()):
    if data is None:
        respond_with_error(status_code=400, detail="Form inputs not found")

    result = iterate_impacts(data())

    if result is None:
        respond_with_error(status_code=404, detail="Parameter info category not found")
    else:
        return [impact for impact in result]
        
def image_content_type(outfile: Path) -> str:
    """ Derives a content type from an image file's suffix """
    return f"image/{outfile.suffix[1:]}"

def image_response(filename: str) -> StreamingResponse:
    """
    Generates a bytestream response from a local image file

    Parameters
    ----------
    filename: str
        Path name of the image file on the local file system

    Returns
    -------
    StreamingResponse

    """
    img_file = Path(filename)
    with img_file.open(mode = "rb") as image:
        return StreamingResponse(
                    io.BytesIO(image.read()),
                    media_type = image_content_type(img_file)
            )

@app.get("/api/iteration/image", summary="Scatter Plot")
async def read_iteration_plot(data: IterationQueryData=Depends()):
    """ Scatter plot from iterating the impact map for a specified set of parameters """
    if data is None:
        respond_with_error(status_code=400, detail="Form inputs not found")

    result = iterate_impacts(data())

    if result is None:
        respond_with_error(status_code=404, detail="Parameter info category not found")
    else:
        return image_response(scatter_plot(result))
