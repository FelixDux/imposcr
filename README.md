# Impact Oscillator
## Overview
This project is an opportunity for me to practice some new (for me) programming techniques, while at the same time indulging in a bit of nostalgia by revisiting the research I did for my PhD. I have not kept up to date with developments in the field since I left academia in 1992, so nothing in this project is likely to contribute to current research. Instead my aim is to reproduce the programming aspects of the work I did then, but with the benefit of 3 decades of software engineering experience and using a language and programming techniques which were not available back then.

## Mathematical Background

Link to static page

## Functionality
The software generates graphical plots of the following:

- Scatter plots of iterated applications of the impact map for a given set of parameter values and initial conditions
- Plots of the singularity set on the impact surface
- Domain of attraction plots on the impact surface for competing ![equation](https://latex.codecogs.com/svg.latex?%28%20m%20%2C%20n%29), ![equation](https://latex.codecogs.com/svg.latex?%28%5Cinfty%20%2C%20n%29) and chaotic orbits (WARNING: these can take a long time to run)

Various other interesting plots will come later, time permitting, including:

- ![equation](https://latex.codecogs.com/svg.latex?V_%7Bn%7D%2C%20%5Csigma) response curves for (1, *n*) orbits for a given values of ![equation](https://latex.codecogs.com/svg.latex?%5Comega) and *r*, showing bifurcation points where orbits become dynamically unstable or unphysical (the latter established numerically)
- Time series plots of *x*(*t*) for a given set of parameter values and initial conditions
- The 'stroboscopic' Poincar&#233; map, which samples the displacement and velocity at each forcing cycle
- Plots of the velocity vs. the displacement
- ![equation](https://latex.codecogs.com/svg.latex?V_%7Bn%7D%2C%20%5Comega) response curves for (1, *n*) orbits for fixed ![equation](https://latex.codecogs.com/svg.latex?%5Csigma)
- Numerically-generated sensitivity/bifurcation plots

## Architecture

Plan is as follows:

- Business logic in Rust
- Wrap the Rust in Python using FFI, using [PyO3](https://pyo3.rs/v0.14.1/), with [maturin](https://crates.io/crates/maturin) for distribution.
- Use [FastAPI](https://fastapi.tiangolo.com) to provide a Web API (use [pipenv](https://pipenv.kennethreitz.org/en/latest/) instead of venv?)
- Serve an SPA (borrowed from imposcg) from /static endpoint and redirect the root to this
- just serve up data from rust and plot either in Python or in the SPA in js
- Put it inside Docker
- Deploy using elastic beanstalk
- maybe have a CLI as well (using e.g. [Typer](https://typer.tiangolo.com/) or [Click](https://click.palletsprojects.com/en/8.0.x/))?

(.venv) felixdux@Felix-Dux-MBP imposc % PYTHONPATH=".:../imposclib" python -m pytest

## Project Structure

- `imposclib` (Rust project library)
    - `src`
        - subfolders with business logic
        - `lib.rs`
        - `imposclib\`
            - `__init__.py`
            - `config.py`
            - `imposcr.py`
    - `tests\`
- (`imposc`) FastAPI Project
    - `.venv\`
    - `src\`
    - `static\`
    - `test\`
    - `requirements.txt`
    - `requirements-dev.txt`

Later on consider adding:

- `imposc-cli` (Rust CLI project)
    - `src`
        - `main.rs`

## Vertical Slices

Take the opportunity to try some outside-in TDD. To do this I need to plan out the vertical slices - use the [imposcpp](https://github.com/FelixDux/imposccpp.git) project as a guide for this, but start with something super-simple.

A vertical slice will comprise:

- Some SPA functionality
- An API endpoint (and/or a CLI command?)
- A Python function which the endpoint/CLI command wraps
- Units comprising the Python function
- A Rust function or functions which some of the Python units wrap
- Units comprising each Python function

Given that the SPA will be lifted from another project, and in order to start with something manageable, let's start with the simplest possible slice, starting from an API endpoint. The slices we eventually want to play with are big - iterating the impact map etc. Let's start with symbols and groups (see [imposcg](https://github.com/FelixDux/imposcg.git)).

So ... test criteria look like:

- GET `/api/parameter-info/symbols` returns JSON `{"Properties":[{"Parameter":"frequency","Property":"ω"},{"Parameter":"offset","Property":"σ"},{"Parameter":"phi","Property":"φ"}]}`
- GET `/api/parameter-info/groups` returns JSON `{"Properties":[{"Parameter":"frequency","Property":"System parameters"},{"Parameter":"offset","Property":"System parameters"},{"Parameter":"r","Property":"System parameters"},{"Parameter":"phi","Property":"Initial impact"},{"Parameter":"v","Property":"Initial impact"},{"Parameter":"maxPeriods","Property":"Control parameters"},{"Parameter":"numIterations","Property":"Control parameters"},{"Parameter":"numPoints","Property":"Control parameters"}]}`
- (Equivalent for CLI calls)
- In module `imposcr.py`, call to `parameter_info()` with argument == "symbols"|"groups" returns correct JSON object
- Exactly the same for wrapped Rust function

### FFI Links
- https://depth-first.com/articles/2020/08/03/wrapping-rust-types-as-python-classes/
- http://jakegoulding.com/rust-ffi-omnibus/
- https://michael-f-bryan.github.io/rust-ffi-guide/
- https://github.com/rapodaca/hash_set/

## CI/CD is Slow!
https://faun.pub/optimizing-ci-cd-pipeline-for-rust-projects-gitlab-docker-98df64ae3bc4
https://www.lpalmieri.com/posts/fast-rust-docker-builds/
https://kflansburg.com/posts/rust-continuous-delivery/
https://readrust.net/devops-and-deployment


## Installing and Running

The simplest way to get going is to use the production docker image. `docker-compose up` will launch the Web application and serve it to http://localhost:8000.