#!/bin/bash
python_interpreter=$(which python)

maturin_path=$(which python| xargs dirname)/maturin

# Refresh python dependencies
cd imposc/ && $python_interpreter -m pip install -r requirements-dev.txt && cd -

# Rebuild library 
cd imposclib/ && $maturin_path develop
