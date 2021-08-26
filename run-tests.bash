#!/bin/bash

cd ./imposclib
cargo test
cd -
cd ./imposc
PYTHONPATH=".:../imposclib" .venv/bin/python -m pytest
cd -