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
- Wrap the Rust in Python using FFI, probably with [rustpy](https://github.com/iduartgomez/rustypy) or starting with the pattern in [this article](https://depth-first.com/articles/2020/08/03/wrapping-rust-types-as-python-classes/).
- Use FastAPI and Typer to provide a Web API and a CLI respectively
- Serve an SPA (borrowed from imposcg) from /static endpoint and redirect the root to this
- Put it inside Docker
- Deploy using elastic beanstalk

### FFI Links
https://depth-first.com/articles/2020/08/03/wrapping-rust-types-as-python-classes/
http://jakegoulding.com/rust-ffi-omnibus/
https://michael-f-bryan.github.io/rust-ffi-guide/
https://github.com/rapodaca/hash_set/

### PS
Imposccpp tried a similar trick wrapping C++ in Python but the SPA is served separately. Try using K8s to orchestrate this and deploy it to GCloud?

## Installing and Running

TBD