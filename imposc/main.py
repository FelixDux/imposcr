from fastapi import FastAPI, HTTPException
from adapters import parameter_info

app = FastAPI()


@app.get("/")
async def read_main():
    return {"msg": "Watch this space ..."}

@app.get("/api/parameter-info/{category}")
async def read_parameter_info(category):
    result = parameter_info(category)

    if result is None:
        raise HTTPException(status_code=404, detail="Category not found")
    else:
        return result